{ pkgs ? import <nixpkgs> {}}:
with pkgs;
let
  sources = import ./sources.nix {};
  root = builtins.fetchGit { url = ../.; ref = "HEAD"; };
  fetlock = (callPackage sources.fetlock {});
  osx = darwin.apple_sdk.frameworks;
  
  # extractors just contains exact binaries needed, to reduce
  # closure size by avoiding e.g. bash dependency
  extractors = stdenv.mkDerivation {
    pname = "runix-extract";
    version = "1";
    buildCommand = ''
      mkdir -p "$out/bin"
      cp -a --dereference ${xz}/bin/unxz "$out/bin"
    '';
  };
  selection = fetlock.cargo.load ./lock.nix {
    pkgOverrides = self: [
      (self.overrideAttrs {
        runix = base: {
          RUNIX_EXTRACTORS_BIN="${extractors}/bin";
          src = "${root}/cli";
        };
      })
      (self.addBuildInputs {
        runix = if stdenv.isDarwin then [ osx.Security ] else [];
      })
    ];
  };
in
selection.drvsByName.runix
