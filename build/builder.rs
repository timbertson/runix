// TODO: convert more goals into this script?

use std::env;
use std::path::PathBuf;

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
struct Buildable {
	platform: String,
	name: String,
}

#[derive(Debug)]
struct Target {
	output: PathBuf,
	buildable: Buildable,
}

impl Target {
	pub fn build(self) {
		match &self.buildable.name {
			other => panic!("Unknown target: {:?}", &self.buildable)
		}
	}

	pub fn dependency<S1: ToString, S2: ToString>(&self, name: S1) -> Dependency {
		Dependency::new(self, Buildable {
			platform: self.buildable.platform.clone(),
			name: name.to_string(),
		})
	}
}

#[derive(Debug)]
struct Dependency<'a> {
	parent: &'a Target,
	buildable: Buildable,
	built: bool,
}

impl<'a> Dependency<'a> {
	pub fn new(parent: &'a Target, buildable: Buildable) -> Self {
		Self { parent, buildable, built: false }
	}

	pub fn build(&mut self) {
		if !self.built {
			todo!();
			// self.built = true;
		}
	}

	pub fn contents(mut self) -> String {
		self.build();
		todo!()
	}
}
