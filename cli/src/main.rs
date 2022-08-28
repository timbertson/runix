mod paths;
mod rewrite;
mod cache;

use anyhow::*;
use log::*;
use std::{process::Command, os::unix::{process::CommandExt, fs::symlink}, path::{Path, PathBuf}, env};
use itertools::Itertools;
use std::fs;
use crate::paths::*;

use crate::paths::RuntimePaths;

pub fn main() -> Result<()> {
	env_logger::init_from_env(
		env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

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
	} else {
		let paths = RuntimePaths::from_env()?;
		let mut search_paths = Vec::new();

		let mut arg = Some(first_arg);
		if arg.as_deref() == Some("--cache") {
			let server = cache::Server {
				root: "https://cache.nixos.org/".to_owned(),
			};
			let client = cache::Client {
				servers: vec!(server),
				paths: paths.clone(),
			};
			while arg.as_deref() == Some("--cache") {
				let entry = cache::StoreIdentity::from(args.next().unwrap());
				debug!("Caching: {:?}", &entry);
				client.cache(&entry)?;
				let store_path = client.store_path(&entry);
				info!("Cached: {}", store_path.display());
				
				let bin_path = store_path.join("bin");
				if bin_path.exists() {
					search_paths.push(bin_path);
				}

				arg = args.next();
			}
		}

		if let Some(exe) = arg {
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
			
			let mut full_exe = None;
			if !exe.contains('/') {
				for p in search_paths.iter() {
					let candidate = p.join(&exe);
					let is_match = candidate.exists() && fs::metadata(&candidate).map(|m|
						m.is_file() &&
						paths::util::is_executable(m.permissions())
					).unwrap_or(false);
					if is_match {
						full_exe = Some(candidate);
						break;
					}
				}
			}
			
			#[allow(unstable_name_collisions)]
			let child_path = search_paths.into_iter()
				.map(|p| p.to_string_lossy().to_string())
				.chain(env::var("PATH"))
				.intersperse(":".to_owned())
				.collect::<String>();
			
			let mut cmd = Command::new(full_exe.unwrap_or_else(|| PathBuf::from(exe)));
			cmd.args(args);
			debug!("{:?}", cmd);

			Err(cmd
				.env("DYLD_INSERT_LIBRARIES", inject)
				.env("DYLD_FORCE_FLAT_NAMESPACE", "1")
				.env("PATH", child_path)
				.exec().into())
				
		} else {
			Ok(())
		}
	}
}
