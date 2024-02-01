use anyhow::*;
use log::*;
use std::path::PathBuf;
use std::{fs, str::FromStr, fmt::Display};
use serde::{Serialize, Deserialize};
use filetime::{set_file_mtime, FileTime};

use crate::{serde_from_string, cache::NarInfo};
use crate::paths::*;

const LATEST_VERSION: i32 = 1;
// v0: original runix release
// v1: codesign on all rewritten executable files

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum StoreEntryVersion {
	Latest,
	Historical(i32),
}

impl StoreEntryVersion {
	fn v0() -> Self { Self::Historical(0) }
}

impl From<i32> for StoreEntryVersion {
	fn from(value: i32) -> Self {
		if value == LATEST_VERSION {
			Self::Latest
		} else {
			Self::Historical(value)
		}
	}
}

impl Into<i32> for StoreEntryVersion {
	fn into(self) -> i32 {
		match self {
			Self::Latest => LATEST_VERSION,
			Self::Historical(i) => i,
		}
	}
}

pub struct StoreIdentityNameDisplay<'a>(&'a StoreIdentity);
impl<'a> Display for StoreIdentityNameDisplay<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.pair().1.fmt(f)
	}
}

// The directory name within /nix/store, including both the hash and the name
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StoreIdentity {
	directory: String,
}

serde_from_string!(StoreIdentity);

impl StoreIdentity {
	pub fn new(s: String) -> Result<Self> {
		if !s.contains('/') {
			let (hash, _) = Self::validate_pair(&s)?;
			if hash.len() == 32 {
				return Ok(Self { directory: s })
			}
		}
		Err(Self::invalid(&s))
	}

	fn invalid(s: &str) -> Error {
		anyhow!("Invalid store identity: {}", &s)
	}

	fn validate_pair(directory: &str) -> Result<(&str, &str)> {
		directory.split_once('-').ok_or_else(|| Self::invalid(directory))
	}

	pub fn from_path(p: &str) -> Result<Self> {
		// extract the last path component
		Self::from_str(p.rsplit('/').next().unwrap())
	}
	
	// used only in tests
	#[allow(unused)]
	pub fn unsafe_from(directory: &str) -> Self { Self { directory: directory.to_owned() } }

	pub fn hash(&self) -> &str {
		self.pair().0
	}

	pub fn directory(&self) -> &str {
		&self.directory
	}

	fn pair(&self) -> (&str, &str) {
		// validated upon construction, so this should never fail
		Self::validate_pair(&self.directory).unwrap()
	}

	pub fn display_name(&self) -> StoreIdentityNameDisplay<'_> {
		StoreIdentityNameDisplay(&self)
	}
}

impl TryFrom<String> for StoreIdentity {
	type Error = anyhow::Error;

	fn try_from(directory: String) -> Result<Self> {
		Self::new(directory)
	}
}

impl TryFrom<&str> for StoreIdentity {
	type Error = anyhow::Error;

	fn try_from(directory: &str) -> Result<Self> {
		Self::new(directory.to_owned())
	}
}

impl FromStr for StoreIdentity {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::try_from(s)
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
	
	#[serde(default="StoreEntryVersion::v0")]
	pub version: StoreEntryVersion,
}

#[derive(Debug)]
pub enum MetaError {
	UnsupportedVersion,
	LoadError(anyhow::Error),
}

impl std::fmt::Display for MetaError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnsupportedVersion => f.write_str("Unsupported store entry version"),
			Self::LoadError(e) => e.fmt(f),
		}
	}
}

impl std::error::Error for MetaError {}

impl From<Error> for MetaError {
	fn from(err: Error) -> Self {
		Self::LoadError(err)
	}
}

impl StoreMeta {
	fn path(paths: &RuntimePaths, identity: &StoreIdentity) -> PathBuf {
		paths.meta_path.join(&identity.directory)
	}

	pub fn write(paths: &RuntimePaths, nar_info: &NarInfo<'_>, version: StoreEntryVersion) -> Result<()> {
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
				version,
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

	pub fn load(paths: &RuntimePaths, identity: &StoreIdentity) -> Result<StoreMetaFull, MetaError> {
		let p = Self::path(paths, identity);
		let meta = util::with_file_ctx(|| format!("Loading {}", p.display()), || {
			let contents = fs::read_to_string(&p)?;
			let meta = serde_json::from_str(&contents)?;
			let used_timestamp = FileTime::from_last_modification_time(&fs::metadata(&p)?);
			Ok(StoreMetaFull {
				meta,
				used_timestamp
			})
		})?;

		if meta.is_supported() {
			Result::Ok(meta)
		} else {
			debug!("Cache path exists but version unsupported: {:?}", &p);
			Err(MetaError::UnsupportedVersion)
		}
	}
}

// Contents of the meta file as well as access time
#[derive(Clone, Debug)]
pub struct StoreMetaFull {
	meta: StoreMeta,

	#[allow(dead_code)] // future use
	used_timestamp: FileTime,
}

impl StoreMetaFull {
	pub fn references(&self) -> &Vec<StoreIdentity> {
		&self.meta.references
	}

	pub fn is_supported(&self) -> bool {
		match self.meta.version {
			StoreEntryVersion::Latest => true,
			StoreEntryVersion::Historical(_) => false,
		}
	}
}
