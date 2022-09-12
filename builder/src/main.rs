use std::env;
use std::fs;
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
	pub fn path(&mut self) -> String {
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
	uname.replace('-', "-").to_string()
}

pub fn run(mut cmd: Command) {
	run_ref(&mut cmd);
}

pub fn drop_store_prefix(s: &str) -> &str {
	s.strip_prefix("/nix/store/").unwrap_or_else(|| panic!("Not a store path: {}", s))
}


pub fn run_ref(cmd: &mut Command) {
	log!("+ {:?}", cmd);
	match cmd.status().map(|st| st.success()) {
		Ok(true) => (),
		Ok(false) => panic!("Command failed: {:?}", &cmd),
		Err(err) => panic!("Spawn failed: {:?}", &err),
	}
}

pub fn run_output(mut cmd: Command) -> String {
	log!("+ {:?}", cmd);
	cmd.stdout(Stdio::piped());
	String::from_utf8(cmd.output().expect("cmd output").stdout).expect("utf8")
}

impl Target {
	pub fn build(self) {
		let unknown = || panic!("Unknown target: {:?}", &self.buildable);
		match self.buildable.platform.as_str() {
			"all" => {
				// cross-platform targets
				match &self.buildable.name {
					_ => unknown()
				}
			},
			"current" => {
				// pseudo-target, delegate to the real target
				let real = Buildable {
					platform: current_platform(),
					name: self.buildable.name.clone(),
				};
				Dependency::new(real).build();
			},
			_ => {

				// per-platform targets
				match self.buildable.name.as_str() {
					"bootstrap.drv" => {
						self.always_rebuild();
						// TODO: cross-platform docker stuff
						let mut cmd = Command::new("nix-build");
						cmd.arg("../").arg("--out-link").arg(&self.output);
						run(cmd);
						run_ref(Command::new("gup").arg("--contents").arg(&self.output));
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
					
					"bootstrap" => {
						// build archive + push to cachix
						self.dependency(self.tarball_path()).build();
						run_ref(Command::new("cachix").arg("push").arg("runix").arg(self.dependency("bootstrap.drv").read_link()));
					},

					other => {
						// only build a tarball when the tarball name matches the desired platform
						if other == self.tarball_path() {
							let dir = self.dependency("bootstrap.dir").path();
							let contents = fs::read_dir(&dir).expect("readdir").map(|e| e.unwrap().file_name().to_str().unwrap().to_owned());
							run_ref(Command::new("tar")
								.arg("czf").arg(&self.output).args(contents)
								.current_dir(dir)
							);
						} else {
							unknown()
						}
					},
				}
			},
		}
	}
	
	fn tarball_path(&self) -> String {
		format!("runix-{}.tgz", &self.buildable.platform)
	}
	
	pub fn always_rebuild(&self) {
		run_ref(&mut Command::new("gup").arg("--always"));
	}

	pub fn dependency<S: ToString>(&self, name: S) -> Dependency {
		Dependency::new(Buildable {
			platform: self.buildable.platform.clone(),
			name: name.to_string(),
		})
	}
}

#[derive(Debug)]
#[must_use]
struct Dependency {
	buildable: Buildable,
	built: bool,
}

impl Dependency {
	pub fn new(buildable: Buildable) -> Self {
		Self { buildable, built: false }
	}

	pub fn build(&mut self) {
		if !self.built {
			run_ref(Command::new("gup").arg("-u").arg(self.buildable.path()));
			self.built = true;
		}
	}

	pub fn read_link(mut self) -> String {
		fs::read_link(self.path()).expect("readlink").to_string_lossy().to_string()
	}

	pub fn path(&mut self) -> String {
		self.build();
		self.buildable.path()
	}
}
