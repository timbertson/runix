#!/usr/bin/env bash
_() {
	set -eu
	PLATFORM="$(uname -m -s | tr ' ' '-')"
	TMP_TAR="/tmp/runix-bootstrap.tgz"
	TMP_DEST="/tmp/runix-bootstrap"
	FILENAME="bootstrap-$PLATFORM.tgz"
	if [ -n "${LOCAL_BOOTSTRAP:-}" ]; then
		cp "$LOCAL_BOOTSTRAP/$FILENAME" "$TMP_TAR"
	else
		echo "TODO: download"
	fi
	rm -rf "$TMP_DEST"
	mkdir -p "$TMP_DEST"
	tar xzf "$TMP_TAR" -C "$TMP_DEST"
	ln -sfn "$TMP_DEST/store" /tmp/runix
	"$TMP_DEST/runix" --self-install "$TMP_DEST/store-identity"
}
_
