mod rewrite;

use anyhow::*;
use log::*;
use std::{process::Command, os::unix::{process::CommandExt, fs::symlink}, path::Path};
use std::fs;

pub fn main() -> Result<()> {
	env_logger::init();

	let all_args = std::env::args();
	debug!("{:?}", all_args);
	let mut args = all_args.into_iter().skip(1);
	
	let first_arg = args.next().unwrap();
	
	let remap = rewrite::Remap::from_env()?;

	if first_arg == "--transform" {
		let file_arg = args.next().unwrap();
		debug!("rewriting: {:?}", &file_arg);
		if args.next().is_some() {
			return Err(anyhow!("too many arguments"));
		}
		rewrite::rewrite_macos(&file_arg, &remap)
	} else {
		let inject = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/librunix_inject.dylib");

		let tmp_symlink = Path::new(remap.tmp_dest);
		let dest_store = remap.dest_store();
		debug!("Linking {} -> {}", remap.tmp_dest, &dest_store);
		if let Err(_) = symlink(&dest_store, tmp_symlink) {
			debug!("Unlinking {} and retrying ...", remap.tmp_dest);
			fs::remove_file(remap.tmp_dest)?;
			symlink(&dest_store, tmp_symlink)?;
		}

		debug!("Injecting: {:?}", inject);
		let mut cmd = Command::new(first_arg);
		cmd.args(args);
		info!("{:?}", cmd);

		Err(cmd
			.env("DYLD_INSERT_LIBRARIES", inject)
			.env("DYLD_FORCE_FLAT_NAMESPACE", "1")
			.exec().into())
	}
}
