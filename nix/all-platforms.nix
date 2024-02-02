with builtins;
listToAttrs (map (platform: {
	name = platform;
	value = import ./default.nix { inherit platform; };
}) [
	# "x86_64-Darwin" # TODO: https://github.com/timbertson/runix/issues/9
	"x86_64-Linux"
	"arm64-Darwin"
])
