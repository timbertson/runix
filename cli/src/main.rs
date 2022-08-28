mod paths;
mod rewrite;
mod cache;

use anyhow::*;
use log::*;
use std::{process::Command, os::unix::{process::CommandExt, fs::symlink}, path::Path};
use std::fs;
use crate::paths::*;

use crate::paths::RuntimePaths;

pub fn main() -> Result<()> {
	env_logger::init();

	let all_args = std::env::args();
	debug!("{:?}", all_args);
	let mut args = all_args.into_iter().skip(1);
	
	let first_arg = args.next().unwrap();
	
	if first_arg == "--rewrite" {
		let file_arg = args.next().unwrap();
		debug!("rewriting: {:?}", &file_arg);
		if args.next().is_some() {
			return Err(anyhow!("too many arguments"));
		}
		rewrite::rewrite_macos(&file_arg, &RewritePaths::default())
	} else if first_arg == "--cache" {
		let server = cache::Server {
			root: "https://cache.nixos.org/".to_owned(),
		};
		let client = cache::Client {
			servers: vec!(server),
			paths: RuntimePaths::from_env()?,
		};
		for entry in args.map(cache::StoreIdentity::from) {
			debug!("caching: {:?}", &entry);
			client.cache(&entry)?;
			// TODO print path!
		}
		Ok(())
	} else {
		let paths = RuntimePaths::from_env()?;
		let inject = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/librunix_inject.dylib");

		let tmp_symlink = Path::new(paths.rewrite.tmp_dest);
		let dest_store = &paths.store_path;
		// TODO don't bother if it's already correct?
		debug!("Linking {:?} -> {:?}", tmp_symlink, &dest_store);
		if let Err(_) = symlink(&dest_store, tmp_symlink) {
			debug!("Unlinking {:?} and retrying ...", tmp_symlink);
			fs::remove_file(tmp_symlink)?;
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
