./store-paths/*:

Each file contains a store path. This is per-system; i.e. their contents will differ across platforms.

But for a given platform; they should be stable since nixpkgs is pinned.

`gup -u store-paths/foo` will calculate the path (requires nix) but not actually download it. This lets us pass it to runix, and generally the store path won't exist on the host system even if nix is installed.
