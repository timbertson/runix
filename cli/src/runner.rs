use crate::cache::{StoreIdentity, self};
use crate::paths::RuntimePaths;
use crate::platform::Platform;

use std::fs;
use std::collections::HashMap;
use std::path::Path;
use std::io::Write;
use std::env;
use std::path::PathBuf;
use std::os::unix::{process::CommandExt, fs::symlink};
use std::process::Command;

use itertools::Itertools;
use anyhow::*;
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entrypoint {
	derivation: StoreIdentity,
	path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformExec {
	pub exec: Option<Entrypoint>,
	pub requirements: Vec<StoreIdentity>,
}

impl PlatformExec {
	pub fn exec<'a, I: Iterator<Item=String>>(&self, client: &cache::Client, paths: &RuntimePaths, mut args: I) -> Result<()> {
		let tmp_symlink = Path::new(paths.rewrite.tmp_dest);
		let dest_store = &paths.store_path;
		// TODO: don't bother if it's already correct?
		debug!("Linking {:?} -> {:?}", tmp_symlink, &dest_store);
		if let Err(_) = symlink(&dest_store, tmp_symlink) {
			debug!("Unlinking {:?} and retrying ...", tmp_symlink);
			fs::remove_file(tmp_symlink)?;
			symlink(&dest_store, tmp_symlink)?;
		}

		let mut requirement_paths = Vec::new();

		for req in self.requirements.iter() {
			debug!("Caching: {:?}", &req);
			client.cache(&req)?;
			let store_path = paths.store_path_for(&req);
			info!("Cached: {}", store_path.display());
			
			let bin_path = store_path.join("bin");
			if bin_path.exists() {
				requirement_paths.push(bin_path);
			}
		}

		#[allow(unstable_name_collisions)]
		let child_path = requirement_paths.into_iter()
			.map(|p| p.to_string_lossy().to_string())
			.chain(env::var("PATH"))
			.intersperse(":".to_owned())
			.collect::<String>();
			
		let exe = match &self.exec {
			None => PathBuf::from(args.next().ok_or_else(|| anyhow!("Not enough arguments"))?),
			Some(exec) => paths.store_path_for(&exec.derivation).join(&exec.path),
		};

		let mut cmd = Command::new(exe);
		cmd.args(args);
		debug!("{:?}", cmd);

		// let inject = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/librunix_inject.dylib");
		// debug!("Injecting: {:?}", inject);
		
		Err(cmd
			// .env("DYLD_INSERT_LIBRARIES", inject)
			// .env("DYLD_FORCE_FLAT_NAMESPACE", "1")
			.env("RUNIX_ROOT", &paths.runix_root)
			.env("PATH", child_path)
			.exec().into())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RunScript {
	caches: Vec<cache::Server>,
	platform: HashMap<Platform, PlatformExec>,
}

impl RunScript {
	pub fn write<D: Write>(&self, mut dst: D) -> Result<()> {
		write!(&mut dst, "#!/usr/bin/env runix")?;
		serde_json::to_writer_pretty(&mut dst, self)?;
		Ok(())
	}

	pub fn current_platform(&self) -> Result<&PlatformExec> {
		let current = Platform::current()?;
		self.platform.get(&current).ok_or_else(|| anyhow!("No implementations provided for platform [{}]", current.to_string()))
	}

	pub fn load<P: AsRef<Path>>(&self, p: P) -> Result<Self> {
		// TODO load JSON, skipping hashes
		todo!()
	}
}
