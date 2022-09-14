# ./store-paths/*:

Each file contains a store path. This is per-system; i.e. their contents will differ across platforms.

But for a given platform; they should be stable since nixpkgs is pinned.

`gup -u store-paths/foo.drv` will calculate the path (requires nix) but not actually download it. This lets us pass it to runix, and generally the store path won't exist on the host system even if nix is installed.

# ./wrappers/*:

Runscript wrappers built from the corresponding store-path.

# ./platforms/$PLATFORM/$TARGET

Build system for creating the cross-platform bootstrap artifacts.

Useful targets:

 - platforms/current/bootstrap: build archive & upload to cachix
 - platforms/current/release: as above, plus upload release assets
 - platforms/all/release: Only works on OSX, to build all artifacts locally (using docker for other architectures)
