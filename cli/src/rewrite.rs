use anyhow::*;
use std::env;
use log::*;
use std::{process::Command, path::Path, io::BufRead};

// NOTE: these paths must be the same length
const REMAP_SRC:      &'static str = "/nix/store";
const REMAP_TMP_DEST: &'static str = "/tmp/runix";

pub fn rewrite_macos<P: AsRef<Path>>(src_path: &P, remap: &Remap) -> Result<()> {
	let path = src_path.as_ref();

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
			if let Some(replacement) = remap.replacement(word) {
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


pub struct Remap {
	pub src: &'static str,
	pub tmp_dest: &'static str,
	pub dest_prefix: String,
}

impl Remap {
	pub fn from_env() -> Result<Self> {
		let base = env::var("RUNIX_ROOT").map_err(|_| anyhow!("$RUNIX_ROOT required"))?;
		Self::for_dest(base)
	}
	
	pub fn for_dest(dest_prefix: String) -> Result<Self> {
		if !dest_prefix.starts_with("/") {
			return Err(anyhow!("RUNIX_ROOT doesn't begin with / [{}]", &dest_prefix));
		}

		Ok(Self {
			src: REMAP_SRC,
			tmp_dest: REMAP_TMP_DEST,
			dest_prefix: dest_prefix,
		})
	}

	pub fn replacement<'a>(&self, value: &'a str) -> Option<String> {
		let suffix = value.strip_prefix(self.src)?;
		Some(format!("{}{}", &self.tmp_dest, suffix))
	}
	
	pub fn dest_store(&self) -> String {
		format!("{}{}", self.dest_prefix, self.src)
	}
}
