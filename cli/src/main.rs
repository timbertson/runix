mod paths;
mod rewrite;
mod cache;
mod store;
mod platform;
mod runner;
mod serde_util;
mod nix_evaluate;

use std::collections::HashSet;
use std::iter::Peekable;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::env;

use anyhow::*;
use log::*;
use paths::RuntimePaths;
use runner::ScriptType;

use crate::store::{StoreIdentity, StoreMeta};
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

		// once --self-install has fetched the real implementation into the store, it
		// invokes this to activate itself
		Some("--make-current") => {
			args.next();
			let identity = StoreIdentity::new(mandatory_next_arg("--make-current store ID", &mut args)?)?;
			make_current(&paths::RuntimePaths::from_env()?, &identity)
		},

		Some("--merge-into") => {
			args.next();
			let dest = mandatory_next_arg("--merge-into destination path", &mut args)?;
			merge_into(dest, args)
		},

		Some("--dump") => {
			args.next();
			dump_runscripts(args)
		},

		Some("--validate") => {
			args.next();
			validate_runscripts(args)
		},

		// main CLI
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
		references.push(StoreIdentity::new(entry.file_name().to_string_lossy().into_owned())?);
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
	paths::util::symlink_force(&symlink_dest, &current)?;
	info!("Symlinked current runix implementation:\n  {}", symlink_dest.display());
	Ok(())
}

fn install_symlink_to_current_on_path(paths: &RuntimePaths) -> Result<()> {
	let mut current = paths.current_symlink();
	current.push("bin");
	current.push("runix");

	let force_path = env::var("RUNIX_BIN_DEST").ok();
	let mut path = force_path.or_else(|| env::var("PATH").ok())
		.map(|path_str| {
			path_str.split(':')
				.filter(|part| !part.is_empty())
				.map(|dir| dir.to_owned())
				.collect::<Vec<String>>()
		}).unwrap_or_else(|| Vec::new());
	path.sort();
	path.sort_by_key(|p| p.len()); // sort by length, then alphabetically
	
	let bin_path: Vec<PathBuf> = path.into_iter().map(|p| PathBuf::from(p).join("runix")).collect();
	if let Some(existing) = bin_path.iter().find(|p| p.exists()) {
		warn!("Not installing symlink, runix already on $PATH: {}", existing.display());
	} else {
		let mut installed = false;
		for bin_dest in bin_path {
			debug!("Attempting symlink: {:?}", &bin_dest);
			if symlink(&current, &bin_dest).is_ok() {
				installed = true;
				info!("Installed a symlink in {}", bin_dest.display());
				break;
			}
		}

		if !installed {
			warn!("Failed to install anywhere on $PATH, please install manually. e.g:\n$ sudo ln -sfn {} /usr/bin/runix",
				current.display()
			)
		}
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
	let args = vec!("--make-current".to_owned(), store_identity.directory().to_owned());
	script.exec(&paths, platform, args.into_iter())
}

fn merge_into<A: Iterator<Item=String>>(dest: String, components: A) -> Result<()> {
	let mut runscript = None;
	for p in components {
		let component = RunScript::load(p)?;
		match runscript.as_mut() {
			None => runscript = Some(component),
			Some(existing) => existing.merge(component),
		}
	}
	let runscript = runscript.ok_or_else(|| anyhow!("At least one runscript required to merge"))?;
	runscript.write_to(&dest, ScriptType::Standard)
}

fn is_wrapper_script(arg: &str) -> bool {
	let p = Path::new(arg);
	arg.contains('/') && p.exists()
}

fn dump_runscripts<A: Iterator<Item=String>>(script_paths: A) -> Result<()> {
	let paths = paths::RuntimePaths::from_env()?;
	for script_path in script_paths {
		println!("{}", &script_path);
		let runscript = RunScript::load(script_path)?;
		let platform_exec = runscript.get_platform(Platform::current()?)?;
		let client = runscript.client(paths.clone());
		let roots = platform_exec.cache_roots(&client)?;

		for root in roots {
			let mut visited = HashSet::new();
			dump_visit_entrypoint(&paths, &mut visited, 0, root)?;
		}
	}
	Ok(())
}

fn dump_visit_entrypoint(
	paths: &RuntimePaths,
	visited: &mut HashSet<StoreIdentity>,
	indent: usize,
	entry: &StoreIdentity
) -> Result<()> {
	if visited.contains(entry) {
		return Ok(())
	}
	visited.insert(entry.to_owned());
	for i in 0 .. indent {
		if i == (indent-1) {
			print!("+ ");
		} else {
			print!("| ");
		}
	}
	println!("{}", entry.directory());
	let meta = StoreMeta::load(paths, entry)?;
	for child in meta.references() {
		dump_visit_entrypoint(paths, visited, indent + 1, child)?;
	}
	Ok(())
}

fn validate_runscripts<A: Iterator<Item=String>>(script_paths: A) -> Result<()> {
	let paths = paths::RuntimePaths::from_env()?;
	for script_path in script_paths {
		println!("{}", &script_path);
		let runscript = RunScript::load(&script_path)?;
		let client = runscript.client(paths.clone());
		for (_, platform_exec) in runscript.platforms() {
			for root in platform_exec.roots() {
				client.fetch_narinfo(root)?;
				println!("OK: {}", root.directory());
			}
		}
	}
	Ok(())
}

fn default_action<A: Iterator<Item=String>>(mut args: Peekable<A>) -> Result<()> {
	let first_arg = mandatory_arg("at least one", args.peek())?;
	let mut single_platform = Platform::current()?;
	let mut save_to = None;
	let mut script_type = ScriptType::Standard;

	let run_script = if is_wrapper_script(&first_arg) {
		// invoked via shebang
		let script = RunScript::load(first_arg)?;
		args.next();
		script
	} else {
		// Explicit CLI usage
		let mut requirements = vec!();
		let mut entrypoint_arg = None;
		let mut is_multiplatform = false;
		let mut is_expr = false;

		let mut run_script = RunScript::default();

		while let Some(argstr) = args.peek() {
			if argstr == "--help" {
				println!(r#"
RUNNING SOFTWARE:
  runix RUNSCRIPT [...ARGS]
  runix [OPTIONS] STORE_IDENTITY [...CMD]

AUTHORING RUNSCRIPTS:
  runix --save PATH [OPTIONS]
  runix --validate PATH
  runix --dump PATH
  runix --merge-into DEST [...RUNSCRIPTS]

OPTIONS:
--require IDENTITY     Add this store name as a requirement.
--with-cache URI       Add this server to the list of caches used.

OPTIONS for --save:
--auto-bootstrap       Make a runscript self-bootstrapping

--entrypoint VALUE RELPATH
                       Set the entrypoint & path to run. If no entrypoint is given,
                       runix will execute ARGV (after fetching requirments and setting up $PATH).
                       The VALUE is by default a store identity, but can be changed with --expr.
--expr                 Treat the entrypoint VALUE as a nix expression.
--multiplatform        Treat the entrypoint VALUE as a nix expression returning an attrset of
                       `platform: derivation`. Use this when cross-compiling for multiple
                       platforms. Implies `--expr`.
--platforms PLATFORMS  Re-evaluate VALUE for each of the passed-in platforms (comma-separated).
                       Unlike `--multiplatform`, this is used when you have built the same
                       expression natively on multiple platforms.
"#);
				return Ok(());
			} else if argstr == "--require" {
				args.next();
				let entry = StoreIdentity::new(mandatory_next_arg("--require value", &mut args)?)?;
				requirements.push(entry);
			} else if argstr == "--with-cache" {
				args.next();
				let server = cache::Server::from(mandatory_next_arg("--add-cache value", &mut args)?);
				run_script.add_cache(server);
			} else if argstr == "--save" {
				args.next();
				save_to = Some(mandatory_next_arg("--save value", &mut args)?);
			} else if argstr == "--auto-bootstrap" {
				args.next();
				script_type = ScriptType::AutoBootstrap;
			} else if argstr == "--platform" {
				args.next();
				single_platform = Platform::from_str(&mandatory_next_arg("--platform value", &mut args)?)?;
			} else if argstr == "--entrypoint" {
				args.next();
				entrypoint_arg = Some((
					mandatory_next_arg("--entrypoint value", &mut args)?,
					mandatory_next_arg("--entrypoint relpath", &mut args)?
				));
			} else if argstr == "--multiplatform" {
				args.next();
				is_multiplatform = true;
			} else if argstr == "--expr" {
				args.next();
				is_expr = true;
			} else if argstr.starts_with("--") {
				return Err(anyhow!("Unknown option: {}", argstr));
			} else {
				break;
			}
		}

		match entrypoint_arg {
			None => run_script.add_platform(single_platform, PlatformExec { exec: None, requirements })?,
			Some((value, relpath)) => {
				debug!("Computing entrypoint...");
				let mut add_entrypoint = |platform, derivation: StoreIdentity, relpath| {
					run_script.add_platform(platform, PlatformExec {
						requirements: requirements.clone(),
						exec: Some(Entrypoint {
							derivation: StoreIdentity::from(derivation),
							path: relpath,
						}),
					})
				};
				if is_multiplatform {
					for (platform, path) in nix_evaluate::evaluate_multi(&value)? {
						add_entrypoint(Platform::from_str(&platform)?, StoreIdentity::from_path(&path)?, relpath.clone())?;
					}
				} else if is_expr {
					let store_path = nix_evaluate::evaluate_single(&value)?;
					add_entrypoint(single_platform, StoreIdentity::from_path(&store_path)?, relpath)?;
				} else {
					add_entrypoint(single_platform, StoreIdentity::new(value)?, relpath)?;
				}
			}
		};
		run_script
	};

	debug!("Runner script: {:?}", run_script);
	match save_to {
		Some(save_to) => {
			if args.next().is_some() {
				return Err(anyhow!("Too many arguments for --save operation"));
			}
			run_script.write_to(&save_to, script_type)
		},
		None => {
			let paths = paths::RuntimePaths::from_env()?;
			run_script.exec(&paths, single_platform, args)
		},
	}
}
