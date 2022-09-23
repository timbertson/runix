#!bash -eux
gup -u ./nixpkgs-stable.drv
nixpkgs="$(readlink ./nixpkgs-stable.drv)"
pname="$(basename "$2" .drv)"
out_path="$(nix-instantiate --eval "$nixpkgs" -A "$pname".outPath | tr -d '"' | sed -e 's@/nix/store/@@')"
[ -n "$out_path" ]
echo "$out_path" > "$1"
