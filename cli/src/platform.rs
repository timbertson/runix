use std::{str::FromStr, fmt::{Display, self}};

use anyhow::*;

use crate::serde_from_string;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OS {
	Linux,
	macOS,
}

impl FromStr for OS {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"linux" => Ok(Self::Linux),
			"macos" | "darwin" => Ok(Self::macOS),
			other => Err(anyhow!("Unknown OS: {}", other))
		}
	}
}

impl OS {
	pub fn current() -> Result<Self> {
		Self::from_str(std::env::consts::OS)
	}
}

impl Display for OS {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Linux => "Linux",
			Self::macOS => "macOS",
		})
	}
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
	x86_64,
	arm64,
}

impl FromStr for Arch {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"x86_64" => Ok(Self::x86_64),
			"arm64" => Ok(Self::arm64),
			other => Err(anyhow!("Unknown arch: {}", other))
		}
	}
}

impl Arch {
	pub fn current() -> Result<Self> {
		if cfg!(target_arch = "x86_64") {
			Ok(Self::x86_64)
		} else if cfg!(target_arch = "aarch64") {
			Ok(Self::arm64)
		} else {
			Err(anyhow!("Unknown architecture: {}", std::env::consts::ARCH))
		}
	}
}

impl Display for Arch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::x86_64 => "x86_64",
			Self::arm64 => "arm64",
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Platform {
	arch: Arch,
	os: OS,
}

impl Platform {
	pub fn current() -> Result<Self> {
		Ok(Self {
			arch: Arch::current()?,
			os: OS::current()?,
		})
	}
}

impl ToString for Platform {
	fn to_string(&self) -> String {
		format!("{}-{}", self.arch, self.os)
	}
}

impl FromStr for Platform {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let inner: Result<Self> = (|| {
			let mut parts = s.split('-');
			let arch = parts.next();
			let os = parts.next();
			let more = parts.next();
			match (arch, os, more) {
				(Some(arch), Some(os), None) => Ok(Self {
					arch: Arch::from_str(arch)?,
					os: OS::from_str(os)?,
				}),
				_ => Err(anyhow!("Invalid platform"))
			}
		})();
		inner.with_context(||format!("Parsing platform: {}", s))
	}
}

serde_from_string!(Platform);
