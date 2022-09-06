#!bash -eu
gup --always
pname="$(basename "$2")"
gup -u ./store-paths/"$pname"
impl="$(cat ./store-paths/"$pname")"
[ -n "$impl" ]

../target/debug/runix \
	--save "$1" \
	--entrypoint "$impl" bin/"$pname"

chmod +x "$1"
