{ platform ? null }:
(import ./pkgs.nix { inherit platform; }).runix.cli
