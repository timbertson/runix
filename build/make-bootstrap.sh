#!bash -eu
gup --always
gup -u bootstrap-dir
cd bootstrap-dir
tar czf "$1" *
