// Some guidance from: https://fzakaria.com/2021/08/12/a-nix-binary-cache-specification.html

use anyhow::*;
use log::*;
use reqwest::StatusCode;
use reqwest::blocking::Response;
use lazy_static::lazy_static;
use std::{fs, process::{Command, Stdio}, collections::HashSet, path::PathBuf, str::FromStr};

use crate::{paths::RuntimePaths, store::StoreMeta};
use crate::{rewrite, store};
use crate::store::{MetaError, StoreIdentity};
use crate::serde_from_string;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Server {
	pub root: String,
}
serde_from_string!(Server);

impl Server {
	pub fn narinfo_url(&self, entry: &StoreIdentity) -> String {
		// TODO url encode
		format!("{}{}.narinfo", &self.root, entry.hash())
	}

	pub fn nar_url(&self, narinfo: &NarInfo) -> String {
		// TODO url encode
		format!("{}{}", &self.root, &narinfo.url)
	}
}

// TODO use TryFrom to avoid the need to explicitly declare this?
impl FromStr for Server {
	type Err = Error;
	fn from_str(root: &str) -> Result<Self, Self::Err> {
		Ok(From::from(root.to_owned()))
	}
}

impl From<String> for Server {
	fn from(mut root: String) -> Self {
		if !root.ends_with('/') {
			root.push('/');
		}
		Self { root }
	}
}

impl ToString for Server {
	fn to_string(&self) -> String {
		self.root.to_owned()
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Compression {
	XZ,
	Zstd,
}

impl Compression {
	fn parse(s: &str) -> Result<Self> {
		match s {
			"xz" => Ok(Self::XZ),
			"zstd" => Ok(Self::Zstd),
			other => Err(anyhow!("Unknown compression type: {}", other)),
		}
	}
}

#[derive(Clone, Debug)]
pub struct NarInfo<'a> {
	/* Responst example:
	StorePath: /nix/store/p4pclmv1gyja5kzc26npqpia1qqxrf0l-ruby-2.7.3
	URL: nar/1w1fff338fvdw53sqgamddn1b2xgds473pv6y13gizdbqjv4i5p3.nar.xz
	Compression: xz
	FileHash: sha256:1w1fff338fvdw53sqgamddn1b2xgds473pv6y13gizdbqjv4i5p3
	FileSize: 4029176
	NarHash: sha256:1impfw8zdgisxkghq9a3q7cn7jb9zyzgxdydiamp8z2nlyyl0h5h
	NarSize: 18735072
	References: 0d71ygfwbmy1xjlbj1v027dfmy9cqavy-libffi-3.3 0dbbrvlw2rahvzi69bmpqy1z9mvzg62s-gdbm-1.19 0i6vphc3vnr8mg0gxjr61564hnp0s2md-gnugrep-3.6 0vkw1m51q34dr64z5i87dy99an4hfmyg-coreutils-8.32 64ylsrpd025kcyi608w3dqckzyz57mdc-libyaml-0.2.5 65ys3k6gn2s27apky0a0la7wryg3az9q-zlib-1.2.11 9m4hy7cy70w6v2rqjmhvd7ympqkj6yxk-ncurses-6.2 a4yw1svqqk4d8lhwinn9xp847zz9gfma-bash-4.4-p23 hbm0951q7xrl4qd0ccradp6bhjayfi4b-openssl-1.1.1k hjwjf3bj86gswmxva9k40nqx6jrb5qvl-readline-6.3p08 p4pclmv1gyja5kzc26npqpia1qqxrf0l-ruby-2.7.3 sbbifs2ykc05inws26203h0xwcadnf0l-glibc-2.32-46
	Deriver: bidkcs01mww363s4s7akdhbl6ws66b0z-ruby-2.7.3.drv
	Sig: cache.nixos.org-1:GrGV/Ls10TzoOaCnrcAqmPbKXFLLSBDeGNh5EQGKyuGA4K1wv1LcRVb6/sU+NAPK8lDiam8XcdJzUngmdhfTBQ==
	*/
	url: String,
	pub identity: &'a StoreIdentity,
	server: &'a Server,
	compression: Compression,
	pub references: Vec<StoreIdentity>,
}

impl<'a> NarInfo<'a> {
	fn parse(server: &'a Server, identity: &'a StoreIdentity, s: &str) -> Result<Self> {
		let invalid = || anyhow!("Can't parse narinfo response for {:?}:\n{}", identity, s);
		let mut url = None;
		let mut compression = None;
		let mut references = None;

		for line in s.lines() {
			debug!("[narinfo]: {:?}", line);
			let mut words = line.split_whitespace();

			if let Some(key) = words.next() {
				let value_required = || anyhow!("Missing value for key {}", key);
				// let value = |v: Option<&str>| v.ok_or_else(|| anyhow!("Missing value for key {}", key));
				// let value = |mut words: SplitWhitespace| words.next().ok_or_else(|| anyhow!("Missing value for key {}", key));
				match key {
					"URL:" => {
						url = Some(words.next().ok_or_else(value_required)?);
					},
					"Compression:" => {
						compression = Some(Compression::parse(words.next().ok_or_else(value_required)?)?);
					},
					"References:" => {
						references = Some(words.map(StoreIdentity::try_from).collect::<Result<Vec<StoreIdentity>>>()?);
					},
					_ => (),
				}
			}
		}

		Ok(Self {
			url: url.ok_or_else(invalid)?.to_owned(),
			compression: compression.ok_or_else(invalid)?.to_owned(),
			references: references.ok_or_else(invalid)?.to_owned(),
			identity,
			server,
		})
	}
	
	fn nar_url(&self) -> String {
		self.server.nar_url(self)
	}
}


#[derive(Clone, Debug)]
pub struct Client {
	pub servers: Vec<Server>,
	pub paths: RuntimePaths,
}

struct ClientState {
	checked: HashSet<StoreIdentity>,
}

impl Default for ClientState {
	fn default() -> Self {
		ClientState { checked: HashSet::default() }
	}
}

lazy_static! {
	static ref CHECK_ALL_PATHS: bool = std::env::var_os("RUNIX_CHECK").as_deref() == Some(std::ffi::OsStr::new("1"));
}

impl Client {
	pub fn cache(&self, entry: &StoreIdentity) -> Result<()> {
		let mut state = ClientState::default();
		self.paths.with_lock(|| self.cache_with_state(&mut state, entry))
	}
	
	fn cache_with_state(&self, state: &mut ClientState, entry: &StoreIdentity) -> Result<()> {
		if state.checked.contains(entry) {
			// already planned
			Ok(())
		} else {
			state.checked.insert(entry.to_owned());
			let narinfo = self.fetch_narinfo_if_missing(entry)
				.with_context(||format!("Fetching narinfo for {:?}", entry))?;

			if let Some(narinfo) = narinfo {
				// Do dependencies first, we shouldn't write an entry to the sture until it's valid
				for dep in narinfo.references.iter() {
					self.cache_with_state(state, dep)?;
				}
				self.download_and_extract(&narinfo).with_context(|| format!("Caching {:?}", entry))
			} else {
				if *CHECK_ALL_PATHS {
					debug!("Checking cached entry {}", &entry);
					let meta = StoreMeta::load(&self.paths, entry)?;
					for dep in meta.references() {
						self.cache_with_state(state, dep)?;
					}
				}
				StoreMeta::touch(&self.paths, entry)
			}
		}
	}

	pub fn fetch_narinfo<'a>(&'a self, entry: &'a StoreIdentity) -> Result<NarInfo<'a>> {
		for server in self.servers.iter() {
			let url = server.narinfo_url(&entry);
			debug!("fetching {:?}", &url);
			let response = reqwest::blocking::get(&url)?;
			debug!("response: {:?}", response);
			let status = response.status();
			if status.is_success() {
				return Ok(NarInfo::parse(&server, &entry, &response.text()?)?)
			} else {
				if status == StatusCode::NOT_FOUND {
					debug!("Not found");
				} else {
					warn!("Error fetching from {:?}: {:?}", server, status);
					// continue trying other servers, just in case
				}
			}
		}
		Err(anyhow!("Entry {:?} not found on any cache", entry)).context(format!("Servers: {:?}", &self.servers))
	}

	fn fetch_narinfo_if_missing<'a>(&'a self, entry: &'a StoreIdentity) -> Result<Option<NarInfo<'a>>> {
		// NOTE: must hold lock to call this
		let dest_path = self.paths.store_path.join(entry.directory());
		let meta_path = self.paths.meta_path.join(entry.directory());
		let meta = if meta_path.exists() && dest_path.exists() {
			match StoreMeta::load(&self.paths, &entry) {
				Result::Ok(meta) => Ok(Some(meta)),
				Err(MetaError::UnsupportedVersion) => {
					fs::remove_file(&meta_path)?;
					Ok(None)
				},
				Err(MetaError::LoadError(e)) => {
					Err(e)
				},
			}?
		} else {
			None
		};
		
		match meta {
			Some(_) => Ok(None),
			None => self.fetch_narinfo(entry).map(Some),
		}
	}
	
	fn extract(mut response: Response, compression: Compression, extract_to: &PathBuf) -> Result<()> {
		let extractors_root = option_env!("RUNIX_EXTRACTORS_BIN");
		let decompress_bin = match compression {
			Compression::XZ => "unxz",
			Compression::Zstd => "unzstd",
		};

		let mut decompress_cmd = Command::new(match extractors_root {
			Some(root) => PathBuf::from(root).join(decompress_bin),
			None => PathBuf::from(decompress_bin),
		});

		decompress_cmd.stdin(Stdio::piped());
		decompress_cmd.stdout(Stdio::piped());

		debug!("+ {:?}", &decompress_cmd);
		let mut decompress = decompress_cmd.spawn().with_context(|| format!("Spawning {:?}", &decompress_cmd))?;
		let mut decompress_in = decompress.stdin.take().expect("missing pipe");
		let decompress_out = decompress.stdout.take().expect("missing pipe");
		
		let extract_to = extract_to.to_owned();
		let unpack_thread = std::thread::spawn(move || {
			let result: Result<()> = (|| {
				let decoder = nix_nar::Decoder::new(decompress_out)?;
				decoder.unpack(&extract_to)
					.with_context(|| format!("Extracting NAR to {}", extract_to.display()))?;
				Ok(())
			})();
			result
		});

		response.copy_to(&mut decompress_in).context("Writing response body")?;
		drop(decompress_in);

		// TODO shouldn't need unwrap, but the types are perplexing...
		unpack_thread.join().unwrap()?;
		Ok(())
	}

	fn download_and_extract(&self, nar_info: &NarInfo<'_>) -> Result<()> {
		fs::create_dir_all(&self.paths.store_path)?;
		let dest = self.paths.store_path.join(nar_info.identity.directory());
		if dest.exists() {
			// remove previous attempt
			crate::paths::util::rm_recursive(&dest)?;
		}

		let url = nar_info.nar_url();
		debug!("fetching {:?}", &url);
		let response = reqwest::blocking::get(&url)?;
		response.error_for_status_ref().with_context(||format!("Fetching {}", &url))?;
		debug!("fetch response {:?}", &url);
		
		Self::extract(response, nar_info.compression, &dest)
			.with_context(|| format!("Extracting {} into {}", &url, dest.display()))?;

		// rewrite rpaths etc.
		rewrite::rewrite_all_recursively(&dest, &self.paths.rewrite, &nar_info.references)?;
		
		// create the meta file, which signifies the validity of the store path
		StoreMeta::write(&self.paths, nar_info, store::StoreEntryVersion::Latest)?;

		info!("Fetched {}", nar_info.identity.display_name());
		Ok(())
	}
}
