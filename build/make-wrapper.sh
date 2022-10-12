#!bash -eu
gup --always
set -x
gup -u ./runix
pname="$(basename "$2")"
store_path="./store-paths/$pname.drv"
gup -u "$store_path"
impl="$(cat "$store_path")"
[ -n "$impl" ]

# Default runscript:
./runix \
	--save "$1" \
	--entrypoint "$impl" bin/"$pname"

# auto-bootstrap runscript:
./runix \
	--save "$2.bootstrap" \
	--auto-bootstrap \
	--entrypoint "$impl" bin/"$pname"

# multiplatform runscript:
./runix \
	--save "$2.multiplatform" \
	--auto-bootstrap \
	--multiplatform \
	--entrypoint "(import ./multiplatform.nix { attr = \"$pname\"; }).list" bin/"$pname"

# multiplatform runscript (via attrs):
./runix \
	--save "$2.multiplatform-attrs" \
	--auto-bootstrap \
	--multiplatform \
	--entrypoint "(import ./multiplatform.nix { attr = \"$pname\"; }).attrs" bin/"$pname"
