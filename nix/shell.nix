let
	sources = import ./sources.nix {};
	pkgs = import sources.nixpkgs {
		overlays = [
			(import "${sources.fenix}/overlay.nix")
		];
	};
	cachixList = if (builtins.getEnv "USE_SYSTEM_CACHIX" == "1") then [] else [ pkgs.cachix ];
	commonInputs = cachixList ++ (with pkgs; [ gup ]);

	fenix-toolchain = pkgs.fenix.stable.withComponents [ "cargo" "rustc" "rust-src" ];

	# in development, we have tools available to build outside nix
	dev = pkgs.mkShell {
		# RUST_SRC_PATH = "${pkts.rustPlatform.rustLibSrc}";
		buildInputs = commonInputs ++ (with pkgs; [
			fenix-toolchain
			rust-analyzer # IDE
			libiconv # native libs
			curl
			git
			findutils
		] ++
			lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
				frameworks.Security
			])
		);
	};
	
	# in CI, we build the nix expression and then drop into a shell with extra
	# utilities for running tests
	runix = (import ./runix.nix {}).runix;

	ci = pkgs.mkShell {
		# build system will build with cargo; set these to override
		RUNIX_EXE = "${runix.cli}/bin/runix";
		RUNIX_BUILDER_EXE = "${runix.builder}/bin/runix-build";
		RUNIX_TESTS = "${runix.tests}/tests";
		buildInputs = commonInputs ++ [
			runix.cli
			runix.builder
			runix.tests
		];
	};
in


{
	inherit pkgs dev ci;
}
