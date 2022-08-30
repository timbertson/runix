#!bash -eu
gup -u ./nix/nixpkgs-stable.nix ./platform
pname="$(basename "$2")"
out_path="$(nix-instantiate --eval ./nix/nixpkgs-stable.nix -A "$pname".outPath | tr -d '"')"
[ -n "$out_path" ]
echo "$out_path" > "$1"
