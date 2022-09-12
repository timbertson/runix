use crate::cache::{StoreIdentity, self, Server};
use crate::paths::{RuntimePaths, self};
use crate::platform::Platform;

use std::fs;
use std::collections::HashMap;
use std::path::Path;
use std::io::Write;
use std::env;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;
use std::process::Command;

use itertools::Itertools;
use anyhow::*;
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entrypoint {
	pub derivation: StoreIdentity,
	pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformExec {
	pub exec: Option<Entrypoint>,
	pub requirements: Vec<StoreIdentity>,
}

pub fn mandatory_next_arg<'a, I: Iterator<Item=String>>(desc: &'static str, args: &mut I) -> Result<String> {
	mandatory_arg(desc, args.next())
}

pub fn mandatory_arg<T>(desc: &'static str, arg: Option<T>) -> Result<T> {
	arg.ok_or_else(|| anyhow!("Not enough arguments (expecting {})", desc))
}

impl PlatformExec {
	pub fn set_entrypoint(&mut self, entrypoint: Entrypoint) {
		self.exec = Some(entrypoint)
	}
	
	fn install_symlink(paths: &RuntimePaths) -> Result<()> {
		let tmp_symlink = Path::new(paths.rewrite.tmp_dest);
		let dest_store = &paths.store_path;
		// TODO: don't bother if it's already correct?
		debug!("Linking {:?} -> {:?}", tmp_symlink, &dest_store);
		paths::util::symlink_force(&dest_store, tmp_symlink)?;
		Ok(())
	}

	pub fn exec<'a, I: Iterator<Item=String>>(&self, client: &cache::Client, paths: &RuntimePaths, mut args: I) -> Result<()> {
		let mut requirement_paths = Vec::new();

		for req in self.requirements.iter().chain(self.exec.iter().map(|x| &x.derivation)) {
			debug!("Caching: {:?}", &req);
			client.cache(&req)?;
			let store_path = paths.store_path_for(&req);
			debug!("Cached: {}", store_path.display());
			
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
			None => PathBuf::from(mandatory_next_arg("exe path", &mut args)?),
			Some(exec) => paths.store_path_for(&exec.derivation).join(&exec.path),
		};

		let mut cmd = Command::new(exe);
		cmd.args(args);
		debug!("{:?}", cmd);

		// let inject = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/librunix_inject.dylib");
		// debug!("Injecting: {:?}", inject);
		
		Self::install_symlink(paths)?;
		Err(cmd
			// .env("DYLD_INSERT_LIBRARIES", inject)
			// .env("DYLD_FORCE_FLAT_NAMESPACE", "1")
			.env("RUNIX_ROOT", &paths.runix_root)
			.env("PATH", child_path)
			.exec().into())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunScript {
	caches: Vec<cache::Server>,
	platform: HashMap<Platform, PlatformExec>,
}

impl Default for RunScript {
	fn default() -> Self {
		Self { caches: vec!(RunScript::default_cache()), platform: Default::default() }
	}
}

impl RunScript {
	pub fn default_cache() -> Server {
		Server {
			root: "https://cache.nixos.org/".to_owned(),
		}
	}

	pub fn add_cache(&mut self, server: cache::Server) {
		self.caches.push(server)
	}

	pub fn write_to(&self, path: &str) -> Result<()> {
		if path == "-" {
			self.write(std::io::stdout())
		} else {
			let dest = fs::OpenOptions::new()
				.write(true)
				.truncate(true)
				.create(true)
				.open(path)?;
			self.write(dest)?;
			paths::util::ensure_executable(path)
		}
	}

	pub fn write<D: Write>(&self, mut dest: D) -> Result<()> {
		write!(&mut dest, "#!/usr/bin/env runix\n\n")?;
		serde_json::to_writer_pretty(&mut dest, self)?;
		Ok(())
	}
	
	pub fn add_platform(&mut self, platform: Platform, exec: PlatformExec) {
		self.platform.insert(platform, exec);
	}

	pub fn get_platform(&self, platform: Platform) -> Result<&PlatformExec> {
		Self::get_platform_from(platform, &self.platform)
	}

	fn get_platform_from(platform: Platform, impls: &HashMap<Platform, PlatformExec>) -> Result<&PlatformExec> {
		impls.get(&platform).ok_or_else(|| Self::platform_not_found(platform))
	}
	
	fn platform_not_found(platform: Platform) -> Error {
		anyhow!("No implementations provided for platform [{}]", platform.to_string())
	}

	pub fn exec<'a, I: Iterator<Item=String>>(self, platform: Platform, args: I) -> Result<()> {
		let paths = RuntimePaths::from_env()?;
		let Self { caches, platform: platforms } = self;
		let client = cache::Client {
			servers: caches,
			paths: paths.clone(),
		};

		let platform_exec = Self::get_platform_from(platform, &platforms)?;
		platform_exec.exec(&client, &paths, args)
	}

	pub fn load<P: AsRef<Path>>(p: P) -> Result<Self> {
		// Load as JSON, but skip any front-matter
		let path = p.as_ref();
		debug!("Loading: {:?}", path);
		let result: Result<Self> = (|| {
			let entire_file = fs::read_to_string(path)?;
			let idx = entire_file.find("\n{")
				.ok_or_else(|| anyhow!("Missing leading brace"))?;
			let (_, json) = entire_file.split_at(idx);
			
			Ok(serde_json::from_str::<Self>(json)?)
		})();
		result.with_context(|| anyhow!("Loading runix script: {}", path.display()))
	}
	
	pub fn merge(&mut self, other: RunScript) {
		let Self { caches, platform: platforms } = other;
		for cache in caches {
			if !self.caches.contains(&cache) {
				self.caches.push(cache);
			}
		}
		for (platform, platform_exec) in platforms.into_iter() {
			self.platform.insert(platform, platform_exec);
		}
	}
}
