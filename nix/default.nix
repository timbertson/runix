{ platform ? null }:
(import ./pkgs.nix { inherit platform; }).runix.selection.drvsByName.runix
