gup -u ./platform
PLATFORM="$(basename "$(dirname "$2")")"
cd "platforms/$PLATFORM"

function trim_store_path {
	sed -e 's@/nix/store/@@'
}
