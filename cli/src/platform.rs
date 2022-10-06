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
		match s {
			"Linux" /* uname -s */ | "linux" /* consts::OS */ => Ok(Self::Linux),
			"macOS" /* rust name */ | "macos" /* consts::OS */ | "Darwin" /* uname -s */ => Ok(Self::macOS),
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
	i686,
	x86_64,
	aarch64,
}

impl FromStr for Arch {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"i686" => Ok(Self::i686),
			"x86_64" => Ok(Self::x86_64),
			"aarch64" => Ok(Self::aarch64),
			other => Err(anyhow!("Unknown arch: {}", other))
		}
	}
}

impl Arch {
	pub fn current() -> Result<Self> {
		if cfg!(target_arch = "x86_64") {
			Ok(Self::x86_64)
		} else if cfg!(target_arch = "x86") {
			Ok(Self::i686)
		} else if cfg!(target_arch = "aarch64") {
			Ok(Self::i686)
		} else {
			Err(anyhow!("Unknown architecture: {}", std::env::consts::ARCH))
		}
	}
}

impl Display for Arch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::i686 => "i686",
			Self::x86_64 => "x86_64",
			Self::aarch64 => "aarch64",
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Platform {
	os: OS,
	arch: Arch,
}

impl Platform {
	pub fn current() -> Result<Self> {
		Ok(Self {
			os: OS::current()?,
			arch: Arch::current()?,
		})
	}
}

impl ToString for Platform {
	fn to_string(&self) -> String {
		format!("{}-{}", self.os, self.arch)
	}
}

impl FromStr for Platform {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let inner: Result<Self> = (|| {
			let mut parts = s.split('-');
			let os = parts.next();
			let arch = parts.next();
			let more = parts.next();
			match (os, arch, more) {
				(Some(os), Some(arch), None) => Ok(Self {
					os: OS::from_str(os)?,
					arch: Arch::from_str(arch)?,
				}),
				_ => Err(anyhow!("Invalid platform"))
			}
		})();
		inner.with_context(||format!("Parsing platform: {}", s))
	}
}

serde_from_string!(Platform);
