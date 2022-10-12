{ attr }:
let
	nixpkgs = (import ../nix/sources.nix {}).nixpkgs;
	pkgs = import <nixpkgs> {};
	systems = pkgs.lib.systems.examples;
	pkgsForSystem = localSystem: (import nixpkgs { inherit localSystem; });
	forSystem = localSystem: builtins.getAttr attr (pkgsForSystem localSystem);
in
{
	# multiplatforms can be an attrset:
	attrs = {
		linux = forSystem systems.gnu64;
		apple64 = forSystem systems.aarch64-darwin;
		applex86 = forSystem systems.x86_64-darwin;
	};
	
	# or a list:
	list = [
		(forSystem systems.gnu64)
		(forSystem systems.aarch64-darwin)
		(forSystem systems.x86_64-darwin)
	];
}
