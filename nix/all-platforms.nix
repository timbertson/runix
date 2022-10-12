with builtins;
listToAttrs (map (platform: {
	name = platform;
	value = import ./default.nix { inherit platform; };
}) [
	"x86_64-Darwin"
	"x86_64-Linux"
	"aarch64-Darwin"
])
