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

# create an auto-bootstrap version too
./runix \
	--save "$2.bootstrap" \
	--auto-bootstrap \
	--entrypoint "$impl" bin/"$pname"
