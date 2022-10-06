with builtins;
listToAttrs (map (platform: {
	name = platform;
	value = import ./default.nix { inherit platform; };
}) [
	"Darwin-x86_64"
	"Linux-x86_64"
	"Darwin-aarch64"
])
