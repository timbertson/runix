#!bash -eux
gup -u ./nix/nixpkgs-stable.nix
pname="$(basename "$2" .drv)"
out_path="$(nix-instantiate --eval ./nix/nixpkgs-stable.nix -A "$pname".outPath | tr -d '"' | sed -e 's@/nix/store/@@')"
[ -n "$out_path" ]
echo "$out_path" > "$1"
