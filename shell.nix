let
	pkgs = import <nixpkgs> {
	};
in
with pkgs;
mkShell {
	RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
	buildInputs = [
		cargo
		rustc
		rust-analyzer # IDE
		libiconv curl # native libs
		# ] ++ (
		# lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
		# 	frameworks.Security
		# 	frameworks.CoreServices
		# 	frameworks.CoreFoundation
		# 	frameworks.Foundation
		# 	frameworks.AppKit
		];
}
