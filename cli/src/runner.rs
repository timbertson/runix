use std::{io::Write, path::Path, collections::HashMap};

use crate::cache::{StoreIdentity, self};
use crate::platform::Platform;
use anyhow::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entrypoint {
	derivation: StoreIdentity,
	path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlatformExec {
	exec: Option<Entrypoint>,
	requirements: Vec<StoreIdentity>,
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

	pub fn exec(&self, argv: &[String]) -> Result<()> {
		Ok(())
	}

	pub fn load<P: AsRef<Path>>(&self, p: P) -> Result<Self> {
		// TODO load JSON, skipping hashes
		todo!()
	}
}
