mod github;

use std::env;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use runix_build::*;

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

pub fn all_platforms() -> Vec<&'static str> {
	vec!(
		"Linux-x86_64",
		"Darwin-x86_64",
		"Darwin-aarch64",
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

					"release" => {
						// release all platform assets
						for platform in all_platforms() {
							self.platform_dependency(platform, "release").build();
						}

						// and also release the runscript:
						let runscript_path = self.dependency("runscript").path();
						github::Release::fetch("bootstrap").replace_asset(&runscript_path);
					},

					"runscript" => {
						for platform in all_platforms() {
							self.platform_dependency(platform, "bootstrap").build();
						}
						let wrapper_scripts = all_platforms().into_iter().map(|platform| {
							self.platform_dependency(platform, "bootstrap.dir").path() + "/wrapper"
						});
						run_ref(Command::new(runix_exe())
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
						github::Release::fetch("bootstrap")
							.replace_asset(&self.dependency(self.tarball_name())._path);
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
						run_ref(Command::new(runix_exe()).arg("--generate-bootstrap").arg(&store_path));
						run_ref(Command::new(runix_exe())
							.arg("--save").arg(self.output.join("wrapper"))
							.arg("--platform").arg(&self.buildable.platform)
							.arg("--with-cache").arg("https://runix.cachix.org")
							.arg("--entrypoint").arg(store_identity).arg("bin/runix")
						);
						run_ref(Command::new("ln")
							.arg("-sfn").arg(format!("store/{}/bin/runix", store_identity))
							.arg(self.output.join("runix"))
						);
						
						// Some tar implementations choke trying to extract a file into a readonly
						// dir, so don't do that!
						run_ref(Command::new("chmod").arg("-R").arg("+w").arg(&store_path));
					},

					"bootstrap.drv" => {
						self.always_rebuild();
						let mut cmd = Command::new("nix-build");

						if self.buildable.platform == current_platform() {
							log!("Building current platform");
							cmd.arg("--arg").arg("platform").arg("null");
						} else {
							log!("Cross-building for {:}", &self.buildable.platform);
							cmd.arg("--argstr").arg("platform").arg(&self.buildable.platform);
						}
						cmd.arg("../").arg("--out-link").arg(&self.output);
						run(cmd);
						run_input_ref(&self.output.to_str().unwrap(), Command::new("gup").arg("--contents"));
					},

					_ => unknown(),
				}
			},
		}
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
