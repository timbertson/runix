use anyhow::*;
use std::{env, path::PathBuf};

use crate::store::StoreIdentity;

// NOTE: these paths must be the same length
const NIX_STORE: &'static str = "/nix/store";
const TMP_RUNIX: &'static str = "/tmp/runix";
// TODO for multi-user support, we could use the first 5 chars of the hash of "runix#USERNAME" in b64

const REWRITE_PATHS: RewritePaths = RewritePaths {
	src: NIX_STORE,
	tmp_dest: TMP_RUNIX,
};

#[derive(Clone, Debug)]
pub struct RewritePaths {
	pub src: &'static str,
	pub tmp_dest: &'static str,
}

// impl RewritePaths {
// 	pub fn rewritten<'a>(&self, value: &'a str) -> Option<String> {
// 		let suffix = value.strip_prefix(self.src)?;
// 		Some(format!("{}{}", &self.tmp_dest, suffix))
// 	}
// }

impl Default for RewritePaths {
	fn default() -> Self {
		REWRITE_PATHS
	}
}

#[derive(Clone, Debug)]
pub struct RuntimePaths {
	pub rewrite: RewritePaths,
	pub runix_root: String,
	pub store_path: PathBuf,
	pub meta_path: PathBuf,
}

impl RuntimePaths {
	pub fn store_path_for(&self, entry: &StoreIdentity) -> PathBuf {
		self.store_path.join(&entry.directory)
	}

	pub fn from_env() -> Result<Self> {
		let base = env::var("RUNIX_ROOT").or_else(|_| {
			env::var("HOME").map(|home| format!("{}/.cache/runix", home))
		}).map_err(|_| anyhow!("HOME or RUNIX_ROOT required"))?;
		Self::for_dest(base)
	}

	pub fn current_symlink(&self) -> PathBuf {
		PathBuf::from(&self.runix_root).join("current")
	}
	
	pub fn for_dest(runix_root: String) -> Result<Self> {
		if !runix_root.starts_with("/") {
			return Err(anyhow!("RUNIX_ROOT doesn't begin with / [{}]", &runix_root));
		}

		let rewrite = RewritePaths::default();
		let store_path = PathBuf::from(format!("{}{}", runix_root, rewrite.src));
		let meta_path = store_path.parent().unwrap().join("info");
		Ok(Self {
			rewrite,
			runix_root,
			store_path,
			meta_path,
		})
	}
}

#[allow(dead_code)]
pub mod util {
	use anyhow::*;
	use log::*;
	use std::os::unix::fs::{PermissionsExt, symlink};
	use std::{fs::{Metadata, self}, path::Path};

	pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
		fs::symlink_metadata(&path).with_context(|| format!("stat: {}", path.as_ref().display()))
	}

	pub fn symlink_force<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> Result<()> {
		let original = original.as_ref();
		let link = link.as_ref();
		let attempt = || {
			symlink(original, link)
		};

		attempt().or_else(|_| {
			fs::remove_file(link)?;
			attempt()
		}).with_context(|| format!("Symlinking {} -> {}", link.display(), original.display()))
	}


	pub fn ensure_writeable_stat<P: AsRef<Path>>(path: P, stat: &Metadata) -> Result<()> {
		let mut perms = stat.permissions();
		let mode = perms.mode();
		let writeable = mode | 0o200;
		if mode != writeable {
			debug!("making writeable: {:?}", path.as_ref());
			perms.set_mode(writeable);
			fs::set_permissions(path, perms)?;
		}
		Ok(())
	}

	pub fn ensure_writeable<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		ensure_writeable_stat(path, &symlink_metadata(path)?)
	}

	pub fn ensure_executable<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		let stat = symlink_metadata(path)?;
		let mut perms = stat.permissions();
		let mode = perms.mode();
		let executable = mode | 0o111;
		if mode != executable {
			debug!("making executable: {:?}", path);
			perms.set_mode(executable);
			fs::set_permissions(path, perms)?;
		}
		Ok(())
	}

	pub fn ensure_unwriteable<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		let stat = symlink_metadata(path)?;
		let mut perms = stat.permissions();
		let mode = perms.mode();
		let unwriteable = mode & !0o222;
		if mode != unwriteable {
			debug!("making unwriteable: {:?}", path);
			perms.set_mode(unwriteable);
			fs::set_permissions(path, perms)?;
		}
		Ok(())
	}

	pub fn rm_recursive<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		let stat = symlink_metadata(path)?;
		ensure_writeable_stat(path, &stat)?;
		if stat.is_dir() {
			for entry in fs::read_dir(path)? {
				rm_recursive(entry?.path())?;
			}
			fs::remove_dir(path)?;
		} else {
			fs::remove_file(path)?;
		}
		Ok(())
	}
	
	pub fn is_executable(perms: fs::Permissions) -> bool {
		(perms.mode() & 0o100) != 0
	}
	
}
