{
	platform ? null,
}:
let
	basePkgs = import <nixpkgs> {};
	# convert uname-style platorm into nix platform
	allSystems = (import <nixpkgs/lib>).systems.examples;
	# crossSystem = if platform == null then null else builtins.getAttr platform {
	# 	"Darwin-x86_64" = allSystems.x86_64-darwin;
	# 	"Darwin-aarch64" = allSystems.aarch64-darwin;
	# 	"Linux-x86_64" = allSystems.gnu64;
	# };

	crosSystemRust = if platform == null then {} else {
		rustc.config = builtins.getAttr platform {
			"Darwin-aarch64" = "aarch64-apple-darwin";
		};
	};

	sources = import ./sources.nix {};

	overlay = self: super: with super;
	let
		#  // {
		# 	# convert nix platform into rustc, so rust produces binaries for the target system
		# 	# rustc.config = builtins.getAttr nixPlatform {
		# 	# 	"aarch64-darwin" = "aarch64-apple-darwin";
		# 	# 	# "Linux-x86_64" = "x86_64-unknown-linux-gnu";
		# 	# };
		# };
		
		root = builtins.fetchGit { url = ../.; ref = "HEAD"; };
		fetlock = (callPackage sources.fetlock {});

		runix-rust = self.fenix.combine [
			(self.fenix.stable.withComponents ["cargo" "rustc" "rust-src"])
			(self.fenix.targets.aarch64-apple-darwin.stable.rust-std)
		];

		# extractors just contains exact binaries needed, to reduce
		# closure size by avoiding e.g. bash dependency
		runix-extractors = stdenv.mkDerivation {
			pname = "runix-extract";
			version = "1";
			buildInputs = [ xz ];

			buildCommand = ''
				mkdir -p "$out/bin"
				cp -a --dereference "$(which xz)" "$out/bin"
			'';
		};
		
		commonPkgOverrides = self: [
			(self.overrideAttrs {
				runix = base: {
					RUNIX_EXTRACTORS_BIN="${extractors}/bin";
					src = "${root}/cli";
				};
			})
		];

		makeSelection = fetlock.cargo.load (./lock + "/${if platform == null then "current" else platform}.nix");

		nativeSelection = makeSelection {
			pkgOverrides = commonPkgOverrides;
			overlays = [(self: super: {
				# rustc = runix-rust;
				pkgs = super.pkgs // {
					buildRustCrate = (super.pkgs.buildRustCrate.override {
						rustc = runix-rust;
					});
				};
			})];
		};

		crossSelection = makeSelection {
			overlays = [
				(self: super:
				let
					# buildRustCrateBase = super.pkgs.buildRustCrate.override {
					# 	rustc = runix-rust;
					# };
					buildRustCrateCross = args: (super.pkgs.buildRustCrate.override {
						# fake stdenv just for buildRustCrate
						stdenv = basePkgs.stdenv // {
							hostPlatform = basePkgs.lib.systems.elaborate (allSystems.aarch64-darwin);
						};
						# TODO do we need both?
						# rust = runix-rust;
						rustc = runix-rust;
						
					} (args // {
						# preConfigure = ''
						# export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="${pkgs.pkgsCross.aarch64-darwin.stdenv.cc}/bin/aarch64-apple-darwin-gcc"
						# '';
						# CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER =
						# 	"${pkgs.pkgsCross.aarch64-darwin.stdenv.cc}/bin/aarch64-apple-darwin-gcc";

						extraRustcOpts = ["-C" "linker=${pkgs.pkgsCross.aarch64-darwin.stdenv.cc}/bin/aarch64-apple-darwin-ld"] ++ (
							# if args.pname == "serde_derive" then [
							# 	"-C" "link-args=-L${pkgs.pkgsCross.aarch64-darwin.libiconv}/lib"
							# ] else []

							# []

							if args.pname == "runix" then [
								# TODO not sure why the standard hooks don't catch this? Is it down to cross-platform quirks?
								"-C" "link-args=-F${pkgs.pkgsCross.aarch64-darwin.darwin.apple_sdk.frameworks.Security}/Library/Frameworks"
								"-C" "link-args=-F${pkgs.pkgsCross.aarch64-darwin.darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks"
								"-C" "link-args=-L${pkgs.pkgsCross.aarch64-darwin.libiconv}/lib"
							] else []
						);
					}));
				in
				{
					# rustc = runix-rust;
					# rustc = crossPkgs.rustc;
					
					# override the args to buildRustCrate
					specToDrv = spec:
						if spec.procMacro or false then nativeSelection.specToDrv spec else
						# if spec.pname == "serde_derive" then nativeSelection.specToDrv spec else
						(super.specToDrv spec).override (base: {
							# take buildDependencies from the non-cross version of rust packages
							buildDependencies = map nativeSelection.getDrv (spec.buildDepKeys or []);
						});
					pkgs = super.pkgs // {
						buildRustCrate = buildRustCrateCross;
						# Terrible hack: force specific packages to be native instead of cross.
						# The correct way to do this involves working with splice.nix, but I can't get that working
						# buildRustCrate = args: if lib.elem args.pname [
						# 	# "autocfg"
						# 	"serde_derive" # TODO why isn't this a build input?
						# 	# "quote"
						# 	# "syn"
						# 	# "proc-macro2"
						# 	# "unicode-ident"
						# 	] then buildRustCrateBase args else buildRustCrateCross args;
					};
					# specs = super.specs // {
					# 	autocfg-hostplatform = super.specs."autocfg-1.1.0" // {
					# 	};
					# };
				})
			];
			pkgOverrides = self: [
				(self.addBuildInputs {
					# TODO need to pass in via specTo attrs!
					# indexmap = [ nativeSelection.drvsByName.autocfg ];
					serde_derive = [ pkgs.pkgsCross.aarch64-darwin.libiconv

						# https://github.com/NixOS/nixpkgs/issues/148189
						# = note: libc++abi: terminating with uncaught exception of type std::runtime_error: Failed to spawn codesign_allocate: No such file or directory
						# /nix/store/dwp239sni6pkj0v8xga6q7xvbcdpnpvg-post-link-sign-hook: line 4: 53177 Abort trap: 6
						#				CODESIGN_ALLOCATE=aarch64-apple-darwin-codesign_allocate /nix/store/jps8kgnsykvj75a5vhzhgkd4hvq17q7g-sigtool-0.1.2/bin/codesign -f -s - "$linkerOutput"
						# This doesnt work, but maybe signingUtils will?
						# It's in : /nix/store/hw5cf127y78d7yab2m6x1m43d4fwj2la-nixpkgs/nixpkgs/pkgs/build-support/bintools-wrapper/default.nix
						# (basePkgs.bintools.override { cross.target = "aarch64-apple-darwin"; } )

						# pkgsCross.*.libtapi fails to build; complains that C compiler can't generate a binary.
						# Most likely it's using the target c++ instead of the native one.
						# (pkgs.pkgsCross.aarch64-darwin.darwin.cctools.override { enableTapiSupport = false; })
						(pkgs.pkgsCross.aarch64-darwin.buildPackages.darwin.cctools) #.override { enableTapiSupport = false; })
					];

					runix = [
						(pkgs.pkgsCross.aarch64-darwin.buildPackages.darwin.cctools)
					];
				})

				# (self.addPropagatedBuildInputs {
				# 	security-framework-sys = if stdenv.isDarwin then [
				# 		pkgs.pkgsCross.aarch64-darwin.darwin.apple_sdk.frameworks.Security
				# 		# pkgs.darwin.apple_sdk.frameworks.Security
				# 	] else [];
				# })
				# (self.addBuildInputs {
				# 	runix = if stdenv.isDarwin then [
				# 		pkgs.pkgsCross.aarch64-darwin.darwin.apple_sdk.frameworks.Security
				# 		# pkgs.darwin.apple_sdk.frameworks.Security
				# 	] else [];
				# })
				# (self.overrideSpec {
				# 	indexmap = base: {
				# 		buildDepKeys = [
				# 			("autocfg-hostplatform")
				# 		];
				# 	};
				# })
			] ++ commonPkgOverrides self;
		};
	in {
		inherit runix-rust;
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
	# inherit crossSystem;
	# crossSystem = { system = builtins.currentSystem; }; # // crosSystemRust;
}

