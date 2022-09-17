{
	platform ? null,
}:
let
	sources = import ./sources.nix {};

	overlay = self: super: with super;
	let
		# for the most part we use `super`, which is just the regular nix environment.
		# We could use a cross environment for everything, but our needs are minimal
		# and I couldn't get that building...
		# Our runtime dependencies are minimal, and this saves us having to build everything ourselves
		# (because no cross-compiled binaries are in the binary cache)
		nativePkgs = super;
		
		fenixStable = self.fenix.stable.rustc;

		# cross contains both a `pkgs` package set as well as a rust which can build for the target arch
		cross = if platform == null then {
			pkgs = self;
			rust = fenixStable;
		} else builtins.getAttr platform {
			"Darwin-aarch64" = {
				pkgs = nativePkgs.pkgsCross.aarch64-darwin;
				rust = self.fenix.combine [ fenixStable self.fenix.targets.aarch64-apple-darwin.stable.rust-std ];
			};

			"Darwin-x86_64" = {
				pkgs = nativePkgs.pkgsCross.x86_64-darwin;
				rust = self.fenix.combine [ fenixStable self.fenix.targets.x86_64-apple-darwin.stable.rust-std ];
			};
		};

		root = builtins.fetchGit { url = ../.; ref = "HEAD"; };
		fetlock = (callPackage sources.fetlock {});

		# extractors just contains exact binaries needed, to reduce
		# closure size by avoiding e.g. bash dependency
		runix-extractors = stdenv.mkDerivation {
			pname = "runix-extract";
			version = "1";
			buildCommand = ''
				mkdir -p "$out/bin"
				cp -a --dereference "${cross.pkgs.xz}/bin/xz" "$out/bin"
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

		codesignDeps = lib.optionals stdenv.isDarwin [
			# https://github.com/NixOS/nixpkgs/issues/148189
			# buildpackages are the ones which run on the host system but produce
			(cross.pkgs.buildPackages.darwin.cctools)
		];

		makeSelection = fetlock.cargo.load (./lock + "/${if platform == null then "current" else platform}.nix");

		# everything selected for the build platform
		nativeSelection = makeSelection {
			pkgOverrides = commonPkgOverrides;
		};

		# build for the target platform
		crossSelection = makeSelection {
			overlays = [ # fetlock overlays
				(self: super: {
					specToDrv = spec:
						# Here we re-split deps into build / target. If it's a macro crate,
						# of a build dep, pull it from nativeSelection so we can run it at build time
						# This would happen magically if we used a full cross-env
						if spec.procMacro or false then nativeSelection.specToDrv spec else
							(super.specToDrv spec).override (base: {
								buildDependencies = map nativeSelection.getDrv (spec.buildDepKeys or []);
							});

					pkgs = super.pkgs // {
						buildRustCrate = (
							args: super.pkgs.buildRustCrate.override {
								# fake stdenv just for buildRustCrate. This causes it to pass the relevant --target flag everwhere
								stdenv = nativePkgs.stdenv // {
									hostPlatform = cross.pkgs.hostPlatform;
								};
							} (args // {
								extraRustcOpts = ["-C" "linker=${cross.pkgs.stdenv.cc}/bin/${cross.pkgs.stdenv.cc.targetPrefix}ld"] ++ (
									if args.pname == "runix" then
										# TODO not sure why the standard hooks don't catch this? Is it down to cross-platform quirks?
										[ "-C" "link-args=-L${cross.pkgs.libiconv}/lib" ] ++ (
											if cross.pkgs.stdenv.isDarwin then [
												"-C" "link-args=-F${cross.pkgs.darwin.apple_sdk.frameworks.Security}/Library/Frameworks"
												"-C" "link-args=-F${cross.pkgs.darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks"
											] else []
										)
									else []
								);
							})
						);
					};
				})
			];

			pkgOverrides = self: [
				(self.addBuildInputs {
					serde_derive = [ cross.pkgs.libiconv ] ++ codesignDeps;
					runix = codesignDeps;
				})
			] ++ commonPkgOverrides self;

		};
	in {
		rustc = cross.rust;
		selection = crossSelection.drvsByName;
		nativeSelection = nativeSelection.drvsByName;
		runix = crossSelection.drvsByName.runix;
	};
in

import <nixpkgs> {
	overlays = [
		overlay
		(import "${sources.fenix}/overlay.nix")
	];
}

