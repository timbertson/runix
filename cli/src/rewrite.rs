use anyhow::*;
use log::*;
use memmap2::MmapMut;
use std::{path::Path, fs};
use walkdir::WalkDir;
use crate::{paths::{RewritePaths, self}, store::StoreIdentity};

fn windows_mut_each<T>(v: &mut [T], n: usize, mut f: impl FnMut(&mut [T])) {
	let mut start = 0;
	let mut end = n;
	while end <= v.len()  {
			f(&mut v[start..end]);
			start += 1;
			end += 1;
	}
}

pub struct RewriteReferences {
	len: usize,
	common_prefix: Vec<u8>, // leading prefix of all rewrite_paths
	replacement_prefix: Vec<u8>, // leading prefix of all rewrite_paths
	full_paths: Vec<Vec<u8>>,
}

impl RewriteReferences {
	fn new<'a, I: IntoIterator<Item=&'a StoreIdentity>>(rewrite_paths: &RewritePaths, references: I) -> Option<Self> {
		let common_prefix = rewrite_paths.src.as_bytes().to_owned();
		let replacement_prefix = rewrite_paths.tmp_dest.as_bytes().to_owned();
		assert!(common_prefix.len() == replacement_prefix.len());

		let full_paths: Vec<Vec<u8>> = references.into_iter().map(|r| {
			[&common_prefix, &vec!('/' as u8), r.hash().as_bytes()].concat()
		}).collect();

		let len = match full_paths.first() {
			Some(r) => r.len(),
			None => return None,
		};
		assert!(full_paths.iter().all(|r| r.len() == r.len()));
		Some(Self { len, common_prefix, replacement_prefix, full_paths })
	}
	
	fn replace_exact(&self, slice: &mut [u8]) -> bool {
		let is_match = slice.starts_with(&self.common_prefix) && self.full_paths.iter().any(|c| slice == c);
		if is_match {
			slice[0..self.common_prefix.len()].copy_from_slice(&self.replacement_prefix);
		}
		is_match
	}

	fn replace_all(&self, file: &mut [u8]) -> usize {
		let mut count = 0;
		windows_mut_each(file, self.len, |window| {
			if self.replace_exact(window) {
				count += 1;
			}
		});
		count
	}
}

// TODO support for some kind of opt-out via .nix-support/runix?
pub fn rewrite_all_recursively<'a, P: AsRef<Path>, R: IntoIterator<Item=&'a StoreIdentity>>(
	src_path: &P,
	rewrite_paths: &RewritePaths, references: R) -> Result<()>
{
	let rewrite = match RewriteReferences::new(rewrite_paths, references) {
		None => return Ok(()),
		Some(x) => x,
	};

	for entry in WalkDir::new(src_path).follow_links(false).contents_first(true) {
		let entry = entry?;
		let path = entry.path();
		let stat = entry.metadata()?;
		if stat.is_file() {
			paths::util::ensure_writeable(path)?;
			let file = fs::OpenOptions::new()
				.read(true)
				.write(true)
				.open(&path)?;
			let mut mmap = unsafe { MmapMut::map_mut(&file)? };
			let count = rewrite.replace_all(&mut mmap);
			if count > 0 {
				debug!("Replaced {} items in {:?}", count, path);
			}
		}
		if stat.is_file() || stat.is_dir() {
			paths::util::ensure_unwriteable(&path)?;
		}
		trace!("rewritten recursively: {:?}", path);
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	
	fn setup() -> RewriteReferences {
		RewriteReferences::new(
			&RewritePaths::default(),
			&vec!(
				StoreIdentity::from("abcd-v1".to_owned()),
				StoreIdentity::from("ghij-v1".to_owned()),
			)
		).unwrap()
	}

	#[test]
	fn test_replace_exact() {
		let mut matching_bytes: Vec<u8> = "/nix/store/abcd".bytes().collect();
		let mut mismatch_bytes: Vec<u8> = "/nix/store/zzzz".bytes().collect();
		let rewrite = setup();
		let replaced = rewrite.replace_exact(&mut matching_bytes);
		assert_eq!(String::from_utf8(matching_bytes).unwrap(), "/tmp/runix/abcd");
		assert!(replaced);

		assert_eq!(rewrite.replace_exact(&mut mismatch_bytes), false);
	}

	#[test]
	fn test_replace_all() {
		let mut entire_string: Vec<u8> = r#"
			Take me to the /nix/store/abcd-foo folder!
			I hear /nix/store/ is full of great contents.
			Like /nix/store/ghij :)
		"#.bytes().collect();
		let rewrite = setup();
		let replace_count = rewrite.replace_all(&mut entire_string);
		assert_eq!(String::from_utf8(entire_string).unwrap(), r#"
			Take me to the /tmp/runix/abcd-foo folder!
			I hear /nix/store/ is full of great contents.
			Like /tmp/runix/ghij :)
		"#);
		assert_eq!(replace_count, 2);
	}
}
