{
	platform ? null,
}:
let
	nixPlatform = builtins.getAttr platform {
		"Darwin-aarch64" = "aarch64-apple-darwin";
		"Darwin-x86_64" = "x86_64-apple-darwin";
		"Linux-x86_64" = "x86_64-unknown-linux-gnu";
	};
	sources = import ./sources.nix {};

	overlay = self: super: with super;
	let
		isDarwin = stdenv.hostPlatform.isDarwin;
		fenixStable = self.fenix.stable.withComponents [ "cargo" "rustc" ];

		fenix-rust = if platform == null then fenixStable else self.fenix.combine [
			fenixStable
			(builtins.getAttr nixPlatform self.fenix.targets).stable.rust-std
		];

		root = builtins.fetchGit { url = ../.; ref = "HEAD"; };
		fetlock = (callPackage sources.fetlock {});

		# extractors just contains exact binaries needed, to reduce
		# closure size by avoiding e.g. bash dependency
		runix-extractors = stdenv.mkDerivation {
			pname = "runix-extract";
			version = "1";
			buildCommand = ''
				mkdir -p "$out/bin"
				cp -a --dereference "${xz}/bin/xz" "$out/bin"
			'';
		};
		
		commonPkgOverrides = self: [
			(self.overrideAttrs {
				runix = base: {
					RUNIX_EXTRACTORS_BIN="${runix-extractors}/bin";
					src = "${root}/cli";
				};
			})
		];

		codesignDeps = lib.optionals isDarwin [
			# https://github.com/NixOS/nixpkgs/issues/148189
			# buildpackages are the ones which run on the host system but produce
			(pkgsBuildHost.darwin.cctools)
		];

		frameworkDeps = lib.optionals isDarwin [
			darwin.apple_sdk.frameworks.Security
			darwin.apple_sdk.frameworks.CoreFoundation
		];

		makeSelection = fetlock.cargo.load (./lock + "/${if platform == null then "current" else platform}.nix");

		# a parallel selection universe without any cross-compilation, used for build dependencies
		nativeSelection = makeSelection {
			pkgOverrides = commonPkgOverrides;
			overlays = [
				(self: super: {
					pkgs = super.pkgs // {
						buildRustCrate =
							let
								buildFn = super.pkgs.buildRustCrate.override {
									stdenv = stdenv // { hostPlatform = stdenv.buildPlatform; };
								};
							in args: buildFn (args // {
								preConfigure = preconfigureIconvHack;
								extraRustcOpts = lib.optionals
									(lib.elem args.pname [ "serde_derive" "thiserror-impl" ])
									[ "-C" "link-args=-L${pkgsBuildBuild.libiconv}/lib" ];
							});
					};
				})
			];
		};
		
		# Some build.rs need to link against iconv because `std` crate depends on it.
		# TODO I don't know why it doesn't magically work, since iconv _is_ on the build
		# inputs path. We can't put this in extraRustcOpts since that applies to the
		# project build, but we need to affect the conigure phase (i.e. build.rs).
		# Here's a hack which prepopulates some stuff used by
		# build-rust-crate/configure-crate.nix
		preconfigureIconvHack = ''
			mkdir -p target
			cat <<EOF >> target/link.build
				-C link-args=-L${pkgsBuildBuild.libiconv}/lib
EOF
		'';

		# build for the target platform
		crossSelection = makeSelection {
			overlays = [ # fetlock overlays
				(self: super: {
					specToDrv = spec:
						# Here we re deps into build / target. If it's a macro crate,
						# of a build dep, pull it from nativeSelection so we can run it at build time
						# This would happen magically if we used a full cross-env
						if spec.procMacro or false then nativeSelection.specToDrv spec else
							(super.specToDrv spec).override (base: {
								buildDependencies = map nativeSelection.getDrv (spec.buildDepKeys or []);
							});

					pkgs = super.pkgs // {
						buildRustCrate = args: super.pkgs.buildRustCrate (args // {
							preConfigure = preconfigureIconvHack;
							extraRustcOpts =
								(lib.optionals
									(lib.elem args.pname [ "serde_derive" "thiserror-impl" "runix" ])
									[ "-C" "link-args=-L${pkgsBuildBuild.libiconv}/lib" ]
								) ++
								(lib.optionals
									(args.pname == "runix" )
									# TODO why isn't it enough to add these to buildInputs?
									[
										"-C" "link-args=-F${darwin.apple_sdk.frameworks.Security}/Library/Frameworks"
										"-C" "link-args=-F${darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks"
										"-C" "linker=${stdenv.cc}/bin/${stdenv.cc.targetPrefix}ld"
									]
								) ;
						});
					};
				})
			];

			pkgOverrides = self: [
				(self.addBuildInputs {
					serde_derive = codesignDeps;
					runix = codesignDeps;
				})
			] ++ commonPkgOverrides self;

		};
	in {
		rustc = fenix-rust;
		cargo = fenix-rust;
		selection = crossSelection.drvsByName;
		runix = crossSelection.drvsByName.runix;
	};
in

import <nixpkgs> ({
	overlays = [
		overlay
		(import "${sources.fenix}/overlay.nix")
	];
} // (if platform == null then {} else {
	crossSystem.config = nixPlatform;
}))
