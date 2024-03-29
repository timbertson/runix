use anyhow::*;
use log::*;
use std::{env, path::PathBuf, fs::{self, OpenOptions}};
use fd_lock::RwLock;

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
	pub lock_path: PathBuf,
}

impl RuntimePaths {
	pub fn store_path_for(&self, entry: &StoreIdentity) -> PathBuf {
		self.store_path.join(entry.directory())
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
		let lock_path = PathBuf::from(format!("{}/lock", runix_root));
		Ok(Self {
			rewrite,
			runix_root,
			store_path,
			meta_path,
			lock_path,
		})
	}
	
	pub fn with_lock<T, F: FnOnce() -> Result<T>>(&self, f: F) -> Result<T> {
		fs::create_dir_all(&self.runix_root)?;
		debug!("Acquiring lock {}", self.lock_path.display());
		let mut lockfile = RwLock::new(OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&self.lock_path).context("opening lockfile")?);

		let lock = lockfile.write()?;
		debug!("Acquired lock");
		let result = f();
		debug!("Releasing lock");
		drop(lock);
		result
	}
}

#[allow(dead_code)]
pub mod util {
	use anyhow::*;
	use log::*;
	use std::fmt::Display;
use std::os::unix::fs::{PermissionsExt, symlink};
	use std::{fs::{Metadata, self}, path::Path};

	pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
		fs::symlink_metadata(&path).with_context(|| format!("lstat: {}", path.as_ref().display()))
	}
	
	pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
		let from = from.as_ref();
		let to = to.as_ref();
		fs::rename(from, to).with_context(|| format!("Renaming {} -> {}", from.display(), to.display()))
	}
	
	pub fn with_file_ctx<
		T,
		F: FnOnce() -> Result<T>,
		C: Display + Send + Sync + 'static,
		Ctx: FnOnce() -> C
	>(ctx: Ctx, block: F) -> Result<T> {
		block().with_context(ctx)
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
			fs::set_permissions(&path, perms)
				.with_context(|| format!("adding write permissions to {:?}", path.as_ref()))?;
		}
		Ok(())
	}

	pub fn ensure_writeable<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		ensure_writeable_stat(path, &symlink_metadata(path).context("ensure_writeable")?)
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
			fs::set_permissions(path, perms)
				.with_context(|| format!("adding execute permissions to {:?}", path.display()))?;
		}
		Ok(())
	}

	pub fn ensure_unwriteable<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		let stat = symlink_metadata(path).context("ensure_unwriteable lstat")?;
		let mut perms = stat.permissions();
		let mode = perms.mode();
		let unwriteable = mode & !0o222;
		if mode != unwriteable {
			debug!("making unwriteable: {:?}", path);
			perms.set_mode(unwriteable);
			fs::set_permissions(&path, perms)
				.with_context(|| format!("removing write permissions from {}", path.display()))?;
		}
		Ok(())
	}

	pub fn rm_recursive<P: AsRef<Path>>(path: P) -> Result<()> {
		let path = path.as_ref();
		let stat = symlink_metadata(path).context("rm_recursive lstat")?;
		ensure_writeable_stat(path, &stat)?;
		if stat.is_dir() {
			for entry in fs::read_dir(path)? {
				rm_recursive(entry?.path())?;
			}
			fs::remove_dir(path).with_context(||format!("removing dir {}", path.display()))?;
		} else {
			fs::remove_file(path).with_context(||format!("removing file {}", path.display()))?;
		}
		Ok(())
	}
	
	pub fn is_executable(perms: fs::Permissions) -> bool {
		(perms.mode() & 0o100) != 0
	}
	
}
