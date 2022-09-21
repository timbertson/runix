{ platform ? null }:
let
	nixPlatform = builtins.getAttr platform {
		"Darwin-aarch64" = "aarch64-apple-darwin";
		"Darwin-x86_64" = "x86_64-apple-darwin";
		"Linux-x86_64" = "x86_64-unknown-linux-musl";
	};
	sources = import ./sources.nix {};

	### baseOverlay:
	# provides the base runix overlay, including fenix, wrappers + rust crates
	baseOverlay = self: super: with super;
	let
		isDarwin = stdenv.hostPlatform.isDarwin;
		fenix-rust = self.fenix.stable.withComponents [ "cargo" "rustc" ];
		root = builtins.fetchGit { url = ../.; ref = "HEAD"; };
		fetlock = (callPackage sources.fetlock {});

		# extractors just contains exact binaries needed, to reduce
		# closure size by avoiding e.g. bash dependency
		extractors = stdenv.mkDerivation {
			pname = "runix-extract";
			version = "1";
			buildCommand = ''
				mkdir -p "$out/bin"
				cp -a --dereference "${xz}/bin/xz" "$out/bin/unxz"
			'';
		};
		
		makeSelection = fetlock.cargo.load (./lock + "/${if platform == null then "current" else platform}.nix");
		
		frameworkDeps = lib.optionals isDarwin [
			darwin.apple_sdk.frameworks.Security
			darwin.apple_sdk.frameworks.CoreFoundation
		];

		commonPkgOverrides = api: [
			(api.overrideAttrs {
				runix = base: {
					RUNIX_EXTRACTORS_BIN="${extractors}/bin";
					src = "${root}/cli";
					buildInputs = (super.buildInputs or []) ++ frameworkDeps ++ [ libiconv ];
				};
				webpki-roots = base: {
					buildInputs = (super.buildInputs or []) ++ frameworkDeps ++ [ libiconv ];
				};
			})
		] ++ lib.optionals (stdenv.buildPlatform.isDarwin && stdenv.hostPlatform.isLinux)
		[
			(api.overrideAll (drv: drv.overrideAttrs (base:
				if base.passthru.spec.procMacro or false then {
					# buildRustCrate is not cross-aware, so it tries to specify a .so location when building on
					# darwin and targeting linux. Hack it up so that it's search path finds a .so
					# NOTE: this just stops it blowing up, we also add need to add the .dylib path (done later)
					postFixup = ''
						if [ -e "$lib/lib" ]; then
							pushd $lib/lib
								for lib in *.dylib; do
									# ln -s "$lib" "$(basename "$lib" .dylib)".so
									cp -a "$lib" "$(basename "$lib" .dylib)".so
								done
							popd
						fi
					'';
				} else {}
			)))
		];
		
		selection = makeSelection { pkgOverrides = commonPkgOverrides; };
	in {
		inherit fenix-rust;
		rustc = self.fenix-rust;
		cargo = self.fenix-rust;
		runix = {
			inherit makeSelection selection root commonPkgOverrides isDarwin extractors;

			codesignDeps = lib.optionals isDarwin [
				# https://github.com/NixOS/nixpkgs/issues/148189
				pkgsBuildHost.darwin.cctools
			];
		};
	};

	### crossOverlay:
	# Additional tweaks required to cross-build runix
	crossOverlay = self: super: with super;
	let
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
		
		isBuildingOnDarwinForLinux = stdenv.buildPlatform.isDarwin && stdenv.hostPlatform.isLinux;

		# build for the target platform
		selection = super.runix.makeSelection {
			overlays = [ # fetlock overlays
				(fetlockSelf: fetlockSuper: {
					specToDrv = spec:
						# Here we split deps into build / target. If it's a macro crate,
						# of a build dep, pull it from nativeSelection so we can run it at build time
						# TODO buildRustCrate should handle this ...
						let nativeDrv = key: vanillaPkgs.runix.selection.getDrv key; in
						if spec.procMacro or false then nativeDrv spec.key else
							(fetlockSuper.specToDrv spec).override (base: {
								buildDependencies = map nativeDrv (spec.buildDepKeys or []);
							});

					pkgs = fetlockSuper.pkgs // {
						buildRustCrate = args: fetlockSuper.pkgs.buildRustCrate (args // {
							preConfigure = preconfigureIconvHack;
							extraRustcOpts =
								(lib.optionals
									(lib.elem args.pname [ "runix" "webpki-roots" ])
									[ "-C" "linker=${stdenv.cc}/bin/${stdenv.cc.targetPrefix}ld" ]
								) ++
								(lib.optionals
									(lib.elem args.pname [ "runix" "webpki-roots" ] && super.runix.isDarwin)
									# TODO why isn't it enough to add these to buildInputs?
									[
										"-C" "link-args=-F${darwin.apple_sdk.frameworks.Security}/Library/Frameworks"
										"-C" "link-args=-F${darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks"
									]
								) ++
								
								# Because buildRustPackage isn't cross-aware, it embeds an .so filename instead of .dylib for
								# proc-macro dependencies. Add a second --extern flag to override the first.
								# Note that _just_ this isn't enough; see the above hack where we symlink .so -> .dylib
								(lib.optionals
									(isBuildingOnDarwinForLinux && args.pname == "serde")
									(let impl = vanillaPkgs.runix.selection.drvsByName.serde_derive; in
										[ "--extern" "serde_derive=${impl.lib}/lib/libserde_derive-${impl.metadata}.dylib" ])
								) ++
								(lib.optionals
									(isBuildingOnDarwinForLinux && args.pname == "thiserror")
									(let impl = vanillaPkgs.runix.selection.drvsByName.thiserror-impl; in
										[ "--extern" "thiserror_impl=${impl.lib}/lib/libthiserror_impl-${impl.metadata}.dylib" ])
								);
						});
					};
				})
			];

			pkgOverrides = api: [
				(api.addBuildInputs {
					serde_derive = self.runix.codesignDeps;
					runix = self.runix.codesignDeps;
				})
			] ++ super.runix.commonPkgOverrides api;
		};
	in {
		fenix-rust = self.fenix.combine [
			super.fenix-rust
			(builtins.getAttr nixPlatform self.fenix.targets).stable.rust-std
		];

		runix = super.runix // {
			inherit selection;
			nativeSelection = vanillaPkgs.runix.selection;
		};
	};

	commonOverlays = [
		(import "${sources.fenix}/overlay.nix")
		baseOverlay
	];

	vanillaPkgs = import <nixpkgs> {
		overlays = commonOverlays;
	};

	crossPkgs = if platform == null then vanillaPkgs else import <nixpkgs> {
		crossSystem.config = nixPlatform;
		overlays = commonOverlays ++ [crossOverlay];
	};
in

crossPkgs
