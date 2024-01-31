#!/usr/bin/env bash
_() {
	set -eu
	PLATFORM="$(echo "$(uname -m)-$(uname -s)")"
	TMP_TAR="/tmp/runix-bootstrap.tgz"
	TMP_DEST="/tmp/runix-bootstrap"
	FILENAME="runix-$PLATFORM.tgz"
	if [ -n "${LOCAL_BOOTSTRAP:-}" ]; then
		echo >&2 "[runix-bootstrap] Using local bootstrap archive: $LOCAL_BOOTSTRAP/$FILENAME"
		cp "$LOCAL_BOOTSTRAP/$FILENAME" "$TMP_TAR"
	else
		echo >&2 "[runix-bootstrap] Downloading ${FILENAME} ..."
		curl -o "$TMP_TAR" -sSL "https://github.com/timbertson/runix/releases/download/bootstrap/$FILENAME"
	fi

	function cleanup {
		if [ -e "$TMP_DEST" ]; then
			chmod -R +w "$TMP_DEST"
			rm -rf "$TMP_DEST"
		fi
	}

	cleanup

	echo >&2 "[runix-bootstrap] extracting to ${TMP_DEST} ..."
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
