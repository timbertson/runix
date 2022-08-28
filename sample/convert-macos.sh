#!bash -eu
dest="$1"
entry="$(basename "$2")"
cp -a "./candidate-store/$entry" "$1"
chmod -R u+w "$1"

# for all executable files
find "$1" -perm -u+x -type f | while read exe_file; do
	if file "$exe_file" | grep -q 'Mach-O'; then
		echo "$exe_file"
		../target/debug/runix --rewrite "$exe_file"
	fi
done
