#!/usr/bin/env bash
set -eu
FLAGS="--nocapture --color=always $*"
if [ -n "${RUNIX_TESTS:-}" ]; then
	# inside CI / nix-shell; precompiled tests
	cd ./builder
	for test_exe in $(find "$RUNIX_TESTS" -type f -executable | sort); do
		echo "$test_exe" $FLAGS
		"$test_exe" $FLAGS
	done
else
	cargo test -- $FLAGS
fi

