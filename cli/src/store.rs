use anyhow::*;
use std::path::PathBuf;
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

impl StoreMeta {
	fn path(paths: &RuntimePaths, identity: &StoreIdentity) -> PathBuf {
		paths.meta_path.join(&identity.directory)
	}

	pub fn write(paths: &RuntimePaths, nar_info: &NarInfo<'_>) -> Result<()> {
		let get_dest = || Self::path(paths, nar_info.identity);
		util::with_file_ctx(|| format!("Writing {}", get_dest().display()), || {
			let dest = get_dest();
			fs::create_dir_all(dest.parent().unwrap())?;
			let tmp_dest = dest.with_file_name(format!("{}.tmp", &nar_info.identity.directory));
			let file = fs::OpenOptions::new()
				.write(true)
				.create(true)
				.truncate(true)
				.open(&tmp_dest)
				.with_context(|| format!("Writing {}", tmp_dest.display()))?;
			let store_meta = StoreMeta {
				references: nar_info.references.clone(),
			};
			serde_json::to_writer(file, &store_meta)?;
			util::rename(tmp_dest, dest)?;
			Ok(())
		})
	}

	pub fn touch(paths: &RuntimePaths, identity: &StoreIdentity) -> Result<()> {
		let dest = Self::path(paths, identity);
		util::with_file_ctx(|| format!("updating {}", dest.display()), || {
			Ok(set_file_mtime(&dest, FileTime::now())?)
		})
	}

	pub fn load(paths: &RuntimePaths, identity: &StoreIdentity) -> Result<StoreMetaFull> {
		let p = Self::path(paths, identity);
		util::with_file_ctx(|| format!("Loading {}", p.display()), || {
			let contents = fs::read_to_string(&p)?;
			let meta = serde_json::from_str(&contents)?;
			let used_timestamp = FileTime::from_last_modification_time(&fs::metadata(&p)?);
			Ok(StoreMetaFull {
				meta,
				used_timestamp
			})
		})
	}

}

// Contents of the meta file as well as access time
#[derive(Clone, Debug)]
pub struct StoreMetaFull {
	meta: StoreMeta,
	used_timestamp: FileTime,
}

impl StoreMetaFull {
	pub fn references(&self) -> &Vec<StoreIdentity> {
		&self.meta.references
	}
}
