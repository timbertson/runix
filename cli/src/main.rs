mod paths;
mod rewrite;
mod cache;
mod platform;
mod runner;
mod serde_util;

use std::iter::Peekable;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::{io, fs, env};

use anyhow::*;
use log::*;
use paths::RuntimePaths;

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
	let first_arg = args.peek().map(|s| s.as_str());
	match first_arg {

		// build tooling at release time calls this to pre-rewrite a store
		// (containing only necessary bootstrap entries)
		Some("--generate-bootstrap") => {
			args.next();
			generate_bootstrap(args)
		},

		// bootstrap.sh runs this on the temporary implementation unpacked in /tmp
		Some("--self-install") => {
			args.next();
			self_install(mandatory_next_arg("--self-install RunScript location", &mut args)?)
		},

		// onse --self-install has fetched the real implementation into the store, it
		// invokes this to activate itself
		Some("--make-current") => {
			args.next();
			let identity = StoreIdentity::from(mandatory_next_arg("--make-current store ID", &mut args)?);
			make_current(&paths::RuntimePaths::from_env()?, &identity)
		}

		// public CLI
		_ => default_action(args),
	}
}

fn generate_bootstrap<A: Iterator<Item=String>>(mut args: Peekable<A>) -> Result<()> {
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
}


fn make_current(paths: &RuntimePaths, identity: &StoreIdentity) -> Result<()> {
	let current = paths.current_symlink();

	let symlink_dest = paths.store_path_for(identity);
	if !symlink_dest.exists() {
		return Err(anyhow!("Specified store path does not exist: {}", symlink_dest.display()))
	}
	info!("Marking {} as the current runix implementation", symlink_dest.display());
	symlink(symlink_dest, &current)?;
	Ok(())
}

fn install_symlink_to_current_on_path(paths: &RuntimePaths) -> Result<()> {
	let current = paths.current_symlink();

	let force_path = env::var("RUNIX_BIN_DEST").ok().map(|p| vec!(p));
	let mut path = force_path.unwrap_or_else(|| {
		env::var("PATH").map(|p| {
			p.split(':').filter(|p| !p.is_empty()).map(|s| s.to_owned()).collect::<Vec<String>>()
		}).unwrap_or_else(|_| Vec::new())
	});
	if path.is_empty() {
		path.push("/usr/bin".to_owned());
	}
	path.sort();
	path.sort_by_key(|p| p.len()); // sort by length, then alphabetically
	
	let mut installed = false;
	for p in path.iter() {
		let bin_dest = PathBuf::from(p).join("runix");
		debug!("Attempting symlink: {:?}", &bin_dest);
		if symlink(&current, &bin_dest).is_ok() {
			installed = true;
			info!("Installed a symlink in {}", bin_dest.display());
			break;
		}
	}

	if !installed {
		warn!("Failed to install anywhere on $PATH, please do this yourself, e.g:\n$ sudo ln -sfn {} /usr/bin/runix",
			current.display()
		)
	}
	Ok(())
}

fn self_install(script_path: String) -> Result<()> {
	let paths = paths::RuntimePaths::from_env()?;
	let platform = Platform::current()?;
	let script = RunScript::load(script_path)?;
	let store_identity = &script.get_platform(platform)?.exec.as_ref()
		.ok_or_else(|| anyhow!("Bootstrap is missing an entrypoint"))?
		.derivation;

	// Script looks valid. Make sure there's a symlink on $PATH, then execute it to finish install
	install_symlink_to_current_on_path(&paths)?;
	let args = vec!("--make-current".to_owned(), store_identity.directory.to_owned());
	script.exec(platform, args.into_iter())
}

fn default_action<A: Iterator<Item=String>>(mut args: Peekable<A>) -> Result<()> {
	let first_arg = mandatory_arg("at least one", args.peek())?;
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
		let mut run_script = RunScript::default();

		while let Some(argstr) = args.peek() {
			if argstr == "--help" {
				println!(r#"
USAGE: runix [OPTIONS] [RUNSCRIPT] [...ARGS]

OPTIONS:
--require IDENTITY              Add this store name as a requirement.
--with-cache URI                Add this server to the list of caches used.
--save PATH                     Save a runscript, instead of executing directly.
--entrypoint IDENTITY RELPATH   Set the entrypoint derivation & path to run. If no entrypoint is given,
                                runix will execute ARGS (after fetching requirments and setting up $PATH)."#);
				return Ok(());
			} else if argstr == "--require" {
				args.next();
				let entry = cache::StoreIdentity::from(mandatory_next_arg("--require value", &mut args)?);
				platform_exec.requirements.push(entry);
			} else if argstr == "--with-cache" {
				args.next();
				let server = cache::Server::from(mandatory_next_arg("--add-cache value", &mut args)?);
				run_script.add_cache(server);
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
