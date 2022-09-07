mod paths;
mod rewrite;
mod cache;
mod platform;
mod runner;
mod serde_util;

use std::path::{Path, PathBuf};
use std::{io, fs};

use anyhow::*;
use log::*;

use crate::cache::StoreIdentity;
use crate::paths::RewritePaths;
use crate::platform::Platform;
use crate::runner::{RunScript, PlatformExec, Entrypoint, mandatory_arg, mandatory_next_arg};

pub fn main() -> Result<()> {
	env_logger::init_from_env(
		env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

	let all_args = std::env::args();
	debug!("{:?}", all_args);
	let mut args = all_args.into_iter().skip(1).peekable();
	
	let first_arg = mandatory_arg("at least one", args.peek())?;
	
	if first_arg == "--rewrite-entire-store" { // internal
		args.next();
		let base = PathBuf::from(args.next().unwrap());
		let mut references = Vec::new();
		let mut paths = Vec::new();
		info!("Rewriting store: {:?}", &base);
		for entry in base.read_dir()? {
			let entry = entry?;
			paths.push(entry.path());
			references.push(StoreIdentity::from(entry.file_name().to_string_lossy().into_owned()));
		}

		for path in paths {
			info!("rewriting: {:?} with {} references", &path, references.len());
			rewrite::rewrite_all_recursively(&path, &RewritePaths::default(), &references)?;
		}
		Ok(())
	} else {
		let platform = Platform::current()?;
		let mut save_to = None;
		let script_path = Path::new(&first_arg);

		let run_script = if first_arg.contains('/') && script_path.exists() {
			// invoked via shebang
			let script = RunScript::load(first_arg)?;
			args.next();
			script
		} else {
			// Explicit CLI usage
			let mut platform_exec = PlatformExec {
				exec: None,
				requirements: vec!(),
			};
			let mut entrypoint = None;

			while let Some(argstr) = args.peek() {
				if argstr == "--cache" {
					args.next();
					let entry = cache::StoreIdentity::from(mandatory_next_arg("--cache value", &mut args)?);
					platform_exec.requirements.push(entry);
				} else if argstr == "--save" {
					args.next();
					save_to = Some(mandatory_next_arg("--save value", &mut args)?);
				} else if argstr == "--entrypoint" {
					args.next();
					let derivation = cache::StoreIdentity::from(mandatory_next_arg("--entrypoint derivation", &mut args)?);
					let path = mandatory_next_arg("--entrypoint path", &mut args)?;
					entrypoint = Some(Entrypoint { derivation, path });
				} else {
					break;
				}
			}

			if let Some(entrypoint) = entrypoint {
				platform_exec.set_entrypoint(entrypoint);
			}

			let mut run_script = RunScript::default();
			run_script.add_platform(platform, platform_exec);
			run_script
		};

		debug!("Runner script: {:?}", run_script);
		match save_to {
			Some(save_to) => {
				debug!("Writing to: {}", &save_to);
				if save_to == "-" {
					run_script.write(io::stdout())
				} else {
					let dest = fs::OpenOptions::new()
						.write(true)
						.truncate(true)
						.create(true)
						.open(save_to)?;
					run_script.write(dest)
				}
			},
			None => run_script.exec(platform, args),
		}
	}
}
