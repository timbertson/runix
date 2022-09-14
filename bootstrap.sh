#!/usr/bin/env bash
_() {
	set -eu
	PLATFORM="$(uname -m -s | tr ' ' '-')"
	TMP_TAR="/tmp/runix-bootstrap.tgz"
	TMP_DEST="/tmp/runix-bootstrap"
	FILENAME="runix-$PLATFORM.tgz"
	if [ -n "${LOCAL_BOOTSTRAP:-}" ]; then
		cp "$LOCAL_BOOTSTRAP/$FILENAME" "$TMP_TAR"
	else
		echo >&2 "[runix-bootstrap] Downloading ..."
		curl -o "$TMP_TAR" -sSL "https://github.com/timbertson/runix/releases/download/bootstrap/$FILENAME"
	fi

	function cleanup {
		if [ -e "$TMP_DEST" ]; then
			chmod -R +w "$TMP_DEST"
			rm -rf "$TMP_DEST"
		fi
	}

	cleanup

	echo >&2 "[runix-bootstrap] extracting ..."
	mkdir -p "$TMP_DEST"
	tar xzf "$TMP_TAR" -C "$TMP_DEST"
	rm -f "$TMP_TAR"

	ln -sfn "$TMP_DEST/store" /tmp/runix
	echo >&2 "[runix-bootstrap] Running self-install ..."
	"$TMP_DEST/runix" --self-install "$TMP_DEST/wrapper"

	echo >&2 "[runix-bootstrap] Cleaning up ..."
	cleanup
}
_
