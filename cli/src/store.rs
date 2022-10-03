use anyhow::*;
use std::{fs, str::FromStr, fmt::Display};
use serde::{Serialize, Deserialize};
use filetime::{set_file_mtime, FileTime};

use crate::{serde_from_string, cache::NarInfo};
use crate::paths::*;

pub struct StoreIdentityNameDisplay<'a>(&'a StoreIdentity);
impl<'a> Display for StoreIdentityNameDisplay<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.pair().1.fmt(f)
	}
}

// The directory name within /nix/store, including both the hash and the name
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StoreIdentity {
	pub directory: String,
}

serde_from_string!(StoreIdentity);

impl StoreIdentity {
	pub fn hash(&self) -> &str {
		self.pair().0
	}

	fn pair(&self) -> (&str, &str) {
		self.directory.split_once('-')
			.unwrap_or_else(|| panic!("Invalid store identity: {}", &self.directory))
	}

	pub fn display_name(&self) -> StoreIdentityNameDisplay<'_> {
		StoreIdentityNameDisplay(&self)
	}
}

impl From<String> for StoreIdentity {
	fn from(directory: String) -> Self {
		Self { directory }
	}
}

impl From<&str> for StoreIdentity {
	fn from(directory: &str) -> Self {
		Self::from(directory.to_owned())
	}
}

impl FromStr for StoreIdentity {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::from(s))
	}
}

impl Display for StoreIdentity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.directory.fmt(f)
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreMeta {
	pub references: Vec<StoreIdentity>,
}

// Contents of the meta file as well as access time
#[derive(Clone, Debug)]
pub struct StoreMetaFull {
	meta: StoreMeta,
	used_timesatmp: FileTime,
}

pub fn write_meta(paths: &RuntimePaths, nar_info: &NarInfo<'_>) -> Result<()> {
	let dest = paths.meta_path.join(&nar_info.identity.directory);
	let tmp_dest = dest.with_file_name(format!("{}.tmp", &nar_info.identity.directory));
	let file = fs::OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(&tmp_dest)?;
	let store_meta = StoreMeta {
		references: nar_info.references.clone(),
	};
	serde_json::to_writer(file, &store_meta)?;
	fs::rename(tmp_dest, dest)?;
	Ok(())
}

pub fn touch_meta(paths: &RuntimePaths, identity: &StoreIdentity) -> Result<()> {
	let dest = paths.meta_path.join(&identity.directory);
	set_file_mtime(&dest, FileTime::now())?;
	Ok(())
}
