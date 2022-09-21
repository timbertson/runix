let
	pkgs = import <nixpkgs> {
	};
	sources = import ./nix/sources.nix {};
in
with pkgs;
mkShell {
	RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
	buildInputs = [
		cargo
		rustc
		rust-analyzer # IDE
		libiconv curl # native libs
		cachix

		# gup # TODO: use upstream when v0.8.2 is released
		(let base = pkgs.callPackage "${sources.gup}/nix/gup-python.nix" {};
			in base.overrideAttrs (_: { src = sources.gup; })
		)
		git
	] ++ (
		lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
			frameworks.Security
		]));
}
