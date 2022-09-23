let
	sources = import ./sources.nix {};
	pkgs = import sources.nixpkgs {};
	commonInputs = with pkgs; [
		cachix
		# gup # TODO: use upstream when v0.8.2 is released
		(let base = callPackage "${sources.gup}/nix/gup-python.nix" {};
			in base.overrideAttrs (_: { src = sources.gup; })
		)
	];

	# in development, we have tools available to build outside nix
	dev = pkgs.mkShell {
		# RUST_SRC_PATH = "${pkts.rustPlatform.rustLibSrc}";
		buildInputs = commonInputs ++ (with pkgs; [
			cargo
			rustc
			rust-analyzer # IDE
			libiconv # native libs
			curl
			git
		] ++
			lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
				frameworks.Security
			])
		);
	};
	
	# in CI, we build the nix expression and then drop into a shell with extra
	# utilities for running tests
	runix = (import ./pkgs.nix {}).runix;

	ci = pkgs.mkShell {
		# don't try to rebuild ./build/builder with cargo
		RUNIX_BUILDER_EXE = "${runix.builder}/bin/runix-build";
		RUNIX_TEST_EXE = "${runix.tests}/tests/runix-build";
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
