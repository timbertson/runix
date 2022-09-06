{ pkgs ? import <nixpkgs> {}}:
with pkgs;
let
  sources = import ./sources.nix {};
  root = builtins.fetchGit { url = ../.; };
  fetlock = (callPackage sources.fetlock {});
  osx = darwin.apple_sdk.frameworks;
  selection = fetlock.cargo.load ./lock.nix {
    pkgOverrides = self: [
      (self.overrideAttrs {
        runix = base: { src = "${root}/cli"; };
      })
      (self.addBuildInputs {
        runix = if stdenv.isDarwin then [ osx.Security ] else [];
      })
    ];
  };
in
selection.drvsByName.runix
