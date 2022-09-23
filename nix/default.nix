{ platform ? null }:
(import ./runix.nix { inherit platform; }).runix.cli
