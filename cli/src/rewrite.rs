use anyhow::*;
use log::*;
use std::{process::Command, path::Path, io::BufRead, os::unix::prelude::PermissionsExt};
use walkdir::WalkDir;
use crate::paths::{RewritePaths, self};

pub fn rewrite_recursively<P: AsRef<Path>>(src_path: &P, rewrite_paths: &RewritePaths) -> Result<()> {
	for entry in WalkDir::new(src_path).follow_links(false) {
		let entry = entry?;
		let path = entry.path();
		let stat = entry.metadata()?;
		if stat.is_file() {
			let perms = stat.permissions();
			let mode = perms.mode();
			if (mode & 0o100) != 0 { // user-executable
				// TODO: check magic number(s) myself
				trace!("Checking if executable is a mach-o binary: {:?}", &path);
				if let Result::Ok(file_output) = Command::new("file").arg(path).output() {
					let out_str = String::from_utf8(file_output.stdout)?;
					if out_str.contains("Mach-O") {
						debug!("rewriting mach-o binary: {:?}", path);
						rewrite_macos(path, rewrite_paths)?;
						paths::util::ensure_unwriteable(path)?;
					}
				}
			}
		}
		trace!("rewritten recursively: {:?}", path);
	}

	Ok(())
}

pub fn rewrite_macos<P: AsRef<Path>>(src_path: P, rewrite_paths: &RewritePaths) -> Result<()> {
	let path = src_path.as_ref();
	paths::util::ensure_writeable(path)?;

	/*
	Inspect with `otool -l`
	Sample entry:

	Load command 22
            cmd LC_LOAD_DYLIB
        cmdsize 104
           name /nix/store/mgb9sdhkh8x9hwdpzfs332slxlhmfkp5-libcxx-11.1.0/lib/libc++.1.0.dylib (offset 24)
     time stamp 2 Thu Jan  1 10:00:02 1970
        current version 1.0.0
  compatibility version 1.0.0

  Load command 23
          cmd LC_RPATH
      cmdsize 40
         path /System/Library/Frameworks (offset 12)
	
	Load commands:
	
	LC_LOAD_DYLIB: load a library
	- absolute
	- relative
	- special (starts with @)
		- @executable_path/...
		- @rpath/...
	
	LC_RPATH: add to library search path
	
	To edit, `install_name_tool`
	Usage: install_name_tool [-change old new] ... [-rpath old new] ... [-add_rpath new] ... [-delete_rpath old] ... [-id name] input
	*/

	let mut cmd = Command::new("otool");
	cmd.arg("-l").arg(path);
	debug!("{:?}", cmd);
	let otool_output = cmd.output()?;
	if !otool_output.status.success() {
		warn!("{}", String::from_utf8(otool_output.stderr).unwrap_or_else(|_|"[failed to parse stderr output as utf8]".to_owned()));
		return Err(anyhow!("otool command failed"));
	}
	
	for line_r in otool_output.stdout.lines() {
		let mut replacements = Vec::new();
		let line = line_r?;
		for word in line.split_ascii_whitespace() {
			// TODO be clever about what kind of name it is, we're just assuming it's an LC_LOAD_DYLIB
			if let Some(replacement) = rewrite_paths.rewritten(word) {
				debug!("remapping: {}", &replacement);
				replacements.push((word, replacement));
			}
		}

		if !replacements.is_empty() {
			let mut cmd = Command::new("install_name_tool");
			for (old_value, new_value) in replacements.into_iter() {
				cmd.arg("-change").arg(old_value).arg(new_value);
			}
			cmd.arg(path);
			debug!("{:?}", cmd);
			let status = cmd.spawn()?.wait()?;

			if !status.success() {
				return Err(anyhow!("install_name_tool failed"));
			}
		}
	}
	Ok(())
}
