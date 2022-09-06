mod paths;
mod rewrite;
mod cache;
mod platform;
mod runner;
mod serde_util;

use std::{io, fs};

use anyhow::*;
use log::*;

use crate::platform::Platform;
use crate::runner::{RunScript,PlatformExec};

pub fn main() -> Result<()> {
	env_logger::init_from_env(
		env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

	let all_args = std::env::args();
	debug!("{:?}", all_args);
	let mut args = all_args.into_iter().skip(1);
	
	let first_arg = args.next().unwrap();
	
	if first_arg == "--rewrite" {
		// let file_arg = args.next().unwrap();
		// debug!("rewriting: {:?}", &file_arg);
		// if args.next().is_some() {
		// 	return Err(anyhow!("too many arguments"));
		// }
		// rewrite::rewrite_macos(&file_arg, &RewritePaths::default())
		todo!("remove?");
	} else {
		let mut platform_exec = PlatformExec {
			exec: None,
			requirements: vec!(),
		};
		let mut save_to = None;

		let mut arg = Some(first_arg);
		while let Some(argstr) = arg {
			if argstr == "--cache" {
				let entry = cache::StoreIdentity::from(args.next().unwrap());
				platform_exec.requirements.push(entry);
			} else if argstr == "--save" {
				save_to = Some(args.next().unwrap());
			}
			arg = args.next();
		}

		let platform = Platform::current()?;
		let mut run_script = RunScript::default();
		run_script.add_platform(platform, platform_exec);
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
