mod paths;
mod rewrite;
mod cache;
mod platform;
mod runner;

use anyhow::*;
use log::*;

use crate::{paths::RuntimePaths, runner::PlatformExec};

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
		let paths = RuntimePaths::from_env()?;
		let mut platform_exec = PlatformExec {
			exec: None,
			requirements: vec!(),
		};

		let mut arg = Some(first_arg);
		while arg.as_deref() == Some("--cache") {
			let entry = cache::StoreIdentity::from(args.next().unwrap());
			platform_exec.requirements.push(entry);
			arg = args.next();
		}

		let server = cache::Server {
			root: "https://cache.nixos.org/".to_owned(),
		};
		let client = cache::Client {
			servers: vec!(server),
			paths: paths.clone(),
		};
		platform_exec.exec(&client, &paths, args)
	}
}
