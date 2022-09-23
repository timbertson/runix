#!bash -eu
gup --always
gup -u ./runix
pname="$(basename "$2")"
store_path="./store-paths/$pname.drv"
gup -u "$store_path"
impl="$(cat "$store_path")"
[ -n "$impl" ]

./runix \
	--save "$1" \
	--entrypoint "$impl" bin/"$pname"

chmod +x "$1"
