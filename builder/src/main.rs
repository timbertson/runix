use std::env;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Write;
use std::fmt;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

macro_rules! log {
	($($tts:tt)*) => {
		if env::var("GUP_XTRACE").ok().as_deref() == Some("1") {
			println!($($tts)*);
		}
	}
}

pub fn main() {
	let all_args = std::env::args();
	let mut args = all_args.into_iter();
	log!("{:?}", args);
	args.next();
	let output = args.next().unwrap();
	let target = args.next().unwrap();
	log!("output: {:?}", output);
	log!("target: {:?}", target);
	let mut target_parts = target.strip_prefix("platforms/").unwrap().split('/');
	let platform = target_parts.next().expect("platform");
	let name = target_parts.next().expect("name");
	let extra = target_parts.next();
	if extra.is_some() {
		panic!("Too many parts: {:?}", extra)
	}
	let target = Target {
		output: PathBuf::from(output),
		buildable: Buildable {
			platform: platform.to_owned(),
			name: name.to_owned(),
		}
	};
	target.build()
}

const RUNIX_BIN: &'static str = "../target/debug/runix";

#[derive(Debug)]
struct Buildable {
	platform: String,
	name: String,
}

impl Buildable {
	pub fn path(&self) -> String {
		format!("platforms/{}/{}", &self.platform, &self.name)
	}
}

#[derive(Debug)]
struct Target {
	output: PathBuf,
	buildable: Buildable,
}

pub fn current_platform() -> String {
	let mut cmd = Command::new("uname");
	cmd.arg("-m").arg("-s");
	let uname = run_output(cmd);
	uname.replace(' ', "-").to_string()
}

pub fn run(mut cmd: Command) {
	run_ref(&mut cmd);
}

pub fn drop_store_prefix(s: &str) -> &str {
	s.strip_prefix("/nix/store/").unwrap_or_else(|| panic!("Not a store path: {}", s))
}


fn assert_successful(desc: CmdCensored, status: io::Result<std::process::ExitStatus>) {
	match status.map(|st| st.success()) {
		Ok(true) => (),
		Ok(false) => panic!("Command failed: {:?}", &desc),
		Err(err) => panic!("Spawn failed: {:?}", &err),
	}
}

#[derive(Clone)]
struct CmdCensored {
	argv: Vec<String>,
}

impl<'a> Debug for CmdCensored {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for arg in self.argv.iter() {
			f.write_str(" ")?;
			if arg.contains("Bearer") || arg.contains("Authorization") {
				f.write_str("'*****'")?;
			} else {
				write!(f, "'{}'", arg)?;
			}
		}
		Ok(())
	}
}
fn censor(cmd: &Command) -> CmdCensored {
	CmdCensored {
		argv: std::iter::once(cmd.get_program()).chain(cmd.get_args())
			.map(|s| s.to_string_lossy().to_string())
			.collect(),
	}
}

pub fn run_ref(cmd: &mut Command) {
	log!("+ {:?}", censor(cmd));
	let status = cmd.status();
	assert_successful(censor(cmd), status)
}

pub fn run_output(mut cmd: Command) -> String {
	run_output_ref(&mut cmd)
}

fn readlink_str<P: AsRef<Path>>(p: P) -> String {
	fs::read_link(p.as_ref())
		.unwrap_or_else(|err| panic!("readlink({}): {:?}", p.as_ref().display(), err))
		.to_string_lossy().to_string()
}

pub fn run_output_ref(cmd: &mut Command) -> String {
	let display = censor(cmd);
	log!("+ {:?}", display);
	cmd.stdout(Stdio::piped());
	cmd.stderr(Stdio::inherit());
	let output = cmd.output().expect("cmd output");
	assert_successful(display, Ok(output.status));
	String::from_utf8(output.stdout).expect("utf8").trim_end().to_string()
}


pub fn run_input_ref(input: &str, cmd: &mut Command) {
	let display = censor(cmd);
	log!("+ {:?}", display);
	cmd.stdin(Stdio::piped());

	let mut child = cmd.spawn().unwrap();
	let mut stdin = child.stdin.take().unwrap();
	write!(&mut stdin, "{}", input).unwrap();
	drop(stdin);
	let status = child.wait();
	assert_successful(display, status);
}

pub fn all_platforms() -> Vec<&'static str> {
	vec!(
		"Linux-x86_64",
		"Darwin-x86_64",
		// "Darwin-aarch64", // TODO
	)
}

impl Target {
	pub fn build(self) {
		let unknown = || panic!("Unknown target: {:?}", &self.buildable);
		match self.buildable.platform.as_str() {

			"all" => {

				// cross-platform targets
				match self.buildable.name.as_str() {
					"all" => {
						// alias
						self.dependency("runscript").build()
					},

					"runscript" => {
						for platform in all_platforms() {
							self.platform_dependency(platform, "bootstrap").build()
						}
						let wrapper_scripts = all_platforms().into_iter().map(|platform| {
							self.platform_dependency(platform, "bootstrap.dir").path() + "/wrapper"
						});
						run_ref(Command::new(RUNIX_BIN)
							.arg("--merge-into").arg(&self.output)
							.args(wrapper_scripts)
						);
					},

					other => {
						// try and build platforms/*/$TARGET
						for platform in all_platforms() {
							self.platform_dependency(platform, other).build()
						}
					},
				}
			},

			"current" => {
				// pseudo-target, delegate to the real target
				let real = Buildable {
					platform: current_platform(),
					name: self.buildable.name.clone(),
				};
				Dependency::new(real.path()).build();
			},
			_ => {

				// per-platform targets
				match self.buildable.name.as_str() {

					"release" => {
						// TODO
						// self.dependency("bootstrap").build();
						
						let curl = || {
							let mut c = Command::new("curl");
							c.arg("-H");
							c.arg(format!("Authorization: Bearer {}", env::var("GITHUB_TOKEN").expect("$GITHUB_TOKEN")));
							c.arg("-H").arg("Accept: application/vnd.github+json");
							c
						};
						let release_data = run_output_ref(
							curl().arg("https://api.github.com/repos/timbertson/runix/releases/tags/bootstrap"));
						log!("Release data: {}", &release_data);

						// Rust has JSON parsers, you know...
						let assets_api_url = release_data.split('"')
							.find(|token| token.ends_with("/assets"))
							.expect("assets_url not found");

						let assets_upload_url = release_data.split('"')
							.find(|token| token.contains("/assets{?"))
							.expect("assets_url not found").split("{").next().unwrap();

						// TODO: delete existing assets for this platform

						// new asset
						let tarball = self.dependency(self.tarball_name())._path;
						let result = run_output_ref(curl()
							.arg("-X").arg("POST")
							.arg("-H").arg("Content-Type: application/octet-stream")
							.arg("--data-binary").arg(format!("@{}", tarball))
							.arg(format!("{}?name={}", assets_upload_url, self.tarball_name()))
						);
						dbg!("{}", result);
					},

					"bootstrap" => {
						// build archive + push to cachix
						self.dependency(self.tarball_name()).build();
						run_ref(Command::new("cachix").arg("push").arg("runix").arg(self.dependency("bootstrap.drv").read_link()));
					},
					
					// alias for the tarball name
					"archive" => {
						self.dependency(self.tarball_name()).build();
					},

					other if other == self.tarball_name() => {
						let dir = self.dependency("bootstrap.dir").path();
						let contents = fs::read_dir(&dir).expect("readdir").map(|e| e.unwrap().file_name().to_str().unwrap().to_owned());
						run_ref(Command::new("tar")
							.arg("czf").arg(&self.output).args(contents)
							.current_dir(dir)
						);
					},

					"bootstrap.dir" => {
						// TODO this could be a temporary dir, we only need it to build the .tgz
						let drv = self.dependency("bootstrap.drv").read_link();
						let store_path = self.output.join("store");
						fs::create_dir_all(&store_path).unwrap();
						let mut cmd = Command::new("nix-store");
						cmd.arg("--query").arg("--requisites").arg(&drv);
						for req in run_output(cmd).trim().split('\n') {
							let dest = store_path.join(drop_store_prefix(req));
							run_ref(Command::new("cp").arg("-a").arg(req).arg(dest));
						}

						let store_identity = drop_store_prefix(&drv);
						run_ref(Command::new(RUNIX_BIN).arg("--generate-bootstrap").arg(&store_path));
						run_ref(Command::new(RUNIX_BIN)
							.arg("--save").arg(self.output.join("wrapper"))
							.arg("--with-cache").arg("https://runix.cachix.org")
							.arg("--entrypoint").arg(store_identity).arg("bin/runix")
						);
						run_ref(Command::new("ln")
							.arg("-sfn").arg(format!("store/{}/bin/runix", store_identity))
							.arg(self.output.join("runix"))
						);
					},

					"bootstrap.drv" => {
						self.always_rebuild();
						if self.buildable.platform == current_platform() {
							log!("Building natively");
							let mut cmd = Command::new("nix-build");
							cmd.arg("../").arg("--out-link").arg(&self.output);
							run(cmd);
							run_input_ref(&self.output.to_str().unwrap(), Command::new("gup").arg("--contents"));
						} else {
							log!("Cross-building {} via docker on {}", &self.buildable.platform, current_platform());
							let tools = PathBuf::from(self.dependency("cross-tools.dir").path());
							let nix_drv_entry = drop_store_prefix(&readlink_str(tools.join("nix"))).to_string();
							let uid = run_output_ref(Command::new("id").arg("-u"));
							let cwd = env::current_dir().unwrap();
							let root = cwd.parent().unwrap().to_str().unwrap();
							// TODO: pass in arch, we're just assuming right now
							// Build the base image:
							let docker_image = run_output_ref(Command::new("docker")
								.arg("build")
								.arg("--quiet")
								.arg("--build-arg").arg(format!("HOST_UID={}", &uid))
								.arg("--file").arg("../Dockerfile.builder")
								.arg(".")
							);
							let home = env::var("HOME").unwrap();
							let drv = PathBuf::from(run_output_ref(Command::new("docker")
								.arg("run")
								.arg("--rm")
								.arg("--volume").arg("/nix:/nix")
								.arg("--volume").arg("/etc/nix:/etc/nix")
								.arg("--volume").arg(format!("{}/.cache/runix:/host-runix", home))
								.arg("--volume").arg(format!("{}:/nix-stable", self.nixpkgs_stable()))
								.arg("--volume").arg(format!("{}:/app", root))
								.arg("--env").arg("NIX_PATH=nixpkgs=/nix-stable")
								.arg("--user").arg(uid)
								.arg(&docker_image)
								.arg(format!("/tmp/runix/{}/bin/nix-build", nix_drv_entry))
								.arg("--no-out-link")
								.arg("--argstr").arg("platform").arg(&self.buildable.platform)
								.arg("/app")
							));
							if !drv.exists() {
								panic!("Built derivation does not exist: {}", drv.display());
							}
							symlink(drv, &self.output).unwrap();
						}
					},
					
					"cross-tools.dir" => {
						// Calculate a store path on the target architecture:
						// TODO pass in arch to docker run
						let nixpkgs_stable = self.nixpkgs_stable();
						let cross_store_path = |attr: &str| {
							let store_path = run_output_ref(Command::new("docker")
								.arg("run")
								.arg("--rm")
								.arg("--volume").arg(format!("{}:/nixpkgs-stable", &nixpkgs_stable))
								.arg("nixos/nix")
								.arg("nix-instantiate")
								.arg("--eval")
								.arg("/nixpkgs-stable")
								.arg("--attr")
								.arg(attr)
							);
							if store_path.is_empty() {
								panic!("Empty store path");
							}
							store_path.replace('"', "")
						};
						let nix_drv = cross_store_path("nix.outPath");
						fs::create_dir(&self.output).unwrap();
						symlink(&nix_drv, self.output.join("nix")).unwrap();

						// ensure the platform's nix impl is in the local runix cache
						run_ref(Command::new(RUNIX_BIN)
							.arg("--require").arg(drop_store_prefix(&nix_drv))
							.arg("true")
						);

						// TERRIBLE HACK: some drvs can't be fetched with the linux implementation of
						// nix, because it doesn't expect a case-insensitive
						// filesystem. So pre-fetch them on the host first
						for pname in vec!("stdenv", "ncurses", "perl") {
							let drv = cross_store_path(&format!("{}.drvPath", pname));
							run_ref(Command::new("nix-build").arg("--no-out-link").arg(drv));
						}
					},

					_ => unknown(),
				}
			},
		}
	}

	fn nixpkgs_stable(&self) -> String {
		Dependency::new("./nix/nixpkgs-stable.drv").read_link()
	}
	
	fn tarball_name(&self) -> String {
		format!("runix-{}.tgz", &self.buildable.platform)
	}
	
	pub fn always_rebuild(&self) {
		run_ref(&mut Command::new("gup").arg("--always"));
	}

	pub fn platform_dependency<S1: ToString, S2: ToString>(&self, platform: S1, name: S2) -> Dependency {
		let buildable = Buildable {
			platform: platform.to_string(),
			name: name.to_string(),
		};
		Dependency::new(buildable.path())
	}

	pub fn dependency<S: ToString>(&self, name: S) -> Dependency {
		self.platform_dependency(&self.buildable.platform, name)
	}
}

#[derive(Debug)]
#[must_use]
struct Dependency {
	_path: String,
	built: bool,
}

impl Dependency {
	pub fn new<S: ToString>(path: S) -> Self {
		Self { _path: path.to_string(), built: false }
	}

	pub fn build(&mut self) {
		if !self.built {
			run_ref(Command::new("gup").arg("-u").arg(&self._path));
			self.built = true;
		}
	}

	pub fn read_link(mut self) -> String {
		readlink_str(self.path())
	}

	pub fn path(&mut self) -> String {
		self.build();
		self._path.clone()
	}
}
