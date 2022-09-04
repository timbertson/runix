// Some guidance from: https://fzakaria.com/2021/08/12/a-nix-binary-cache-specification.html

use anyhow::*;
use log::*;
use reqwest::StatusCode;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use std::{fs, process::{Command, Stdio}, collections::HashSet, path::PathBuf};

use crate::paths::RuntimePaths;
use crate::rewrite;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
	pub root: String,
}

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


// The directory name within /nix/store, including both the hash and the name
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StoreIdentity {
	directory: String,
}

impl StoreIdentity {
	pub fn new(directory: String) -> Self {
		Self { directory }
	}

	pub fn hash(&self) -> &str {
		self.directory.split('-').next().unwrap_or_else(|| panic!("empty split"))
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

#[derive(Copy, Clone, Debug)]
pub enum Compression {
	XZ,
}

impl Compression {
	fn parse(s: &str) -> Result<Self> {
		match s {
			"xz" => Ok(Self::XZ),
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
	identity: &'a StoreIdentity,
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
						references = Some(words.map(StoreIdentity::from).collect::<Vec<StoreIdentity>>());
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

impl Client {
	pub fn cache(&self, entry: &StoreIdentity) -> Result<()> {
		let mut state = ClientState::default();
		self.cache_with_state(&mut state, entry)
	}
	
	pub fn store_path(&self, entry: &StoreIdentity) -> PathBuf {
		self.paths.store_path.join(&entry.directory)
	}

	fn cache_with_state(&self, state: &mut ClientState, entry: &StoreIdentity) -> Result<()> {
		if state.checked.contains(entry) {
			// already planned
			Ok(())
		} else {
			state.checked.insert(entry.to_owned());
			if let Some(narinfo) = self.fetch_narinfo_if_missing(entry)? {
				// Do dependencies first, we shouldn't write an entry to the sture until it's valid
				for dep in narinfo.references.iter() {
					self.cache_with_state(state, dep)?;
				}
				self.download_and_extract(&narinfo)
			} else {
				Ok(())
			}
		}
	}

	fn fetch_narinfo_if_missing<'a>(&'a self, entry: &'a StoreIdentity) -> Result<Option<NarInfo<'a>>> {
		let dest_path = self.paths.store_path.join(&entry.directory);
		// TODO: locking for concurrent processes
		if dest_path.exists() {
			debug!("Cache path already exists: {:?}", dest_path);
			return Ok(None);
		}

		for server in self.servers.iter() {
			let url = server.narinfo_url(&entry);
			info!("Caching {:?}", &entry);
			debug!("fetching {:?}", &url);
			let response = reqwest::blocking::get(&url)?;
			debug!("response: {:?}", response);
			let status = response.status();
			if status.is_success() {
				return Ok(Some(NarInfo::parse(&server, &entry, &response.text()?)?))
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
	
	// TODO use nix_nar for smaller closure?
	fn extract(mut response: Response, compression: Compression, extract_to: &PathBuf) -> Result<()> {
		let mut decompress_cmd = match compression {
			// TODO should this be a bundled dependency?
			Compression::XZ => Command::new("unxz"),
		};

		decompress_cmd.stdin(Stdio::piped());
		decompress_cmd.stdout(Stdio::piped());

		debug!("+ {:?}", &decompress_cmd);
		let mut decompress = decompress_cmd.spawn()?;
		let mut decompress_in = decompress.stdin.take().expect("missing pipe");
		let decompress_out = decompress.stdout.take().expect("missing pipe");
		
		let mut extract_cmd = Command::new("nix-store");
		extract_cmd.arg("--restore")
			.arg(&extract_to)
			.stdin(decompress_out);

		debug!("+ {:?}", &extract_cmd);
		let mut extract = extract_cmd.spawn()?;

		response.copy_to(&mut decompress_in)?;
		drop(decompress_in);

		let status = extract.wait()?;
		if !status.success() {
			return Err(anyhow!("nix-store --restore failed"));
		}

		Ok(())
	}

	fn download_and_extract(&self, nar_info: &NarInfo<'_>) -> Result<()> {
		let extract_dest = self.paths.extract_path.join(&nar_info.identity.directory);
		fs::create_dir_all(&self.paths.extract_path)?;
		fs::create_dir_all(&self.paths.store_path)?;
		if extract_dest.exists() {
			// remove previous attempt
			crate::paths::util::rm_recursive(&extract_dest)?;
		}

		let url = nar_info.nar_url();
		debug!("fetching {:?}", &url);
		let response = reqwest::blocking::get(&url)?;
		response.error_for_status_ref()?;
		debug!("fetch response {:?}", &url);
		
		Self::extract(response, nar_info.compression, &extract_dest)?;

		// rewrite rpaths etc.
		// TODO: capture a rewrite_version, so we can redo old paths if rewrite logic changes
		rewrite::rewrite_all_recursively(&extract_dest, &self.paths.rewrite, &nar_info)?;

		let dest = self.paths.store_path.join(&nar_info.identity.directory);

		fs::rename(&extract_dest, &dest)
			.with_context(|| format!("moving {:?} -> {:?}", &extract_dest, &dest))?;

		Ok(())
	}
}
