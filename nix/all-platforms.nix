let platform = p: import ./default.nix { platform = p; }; in
[
	(platform "Darwin-x86_64")
	(platform "Darwin-aarch64")
	(platform "Linux-x86_64")
]
