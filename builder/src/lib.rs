use std::env;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Write;
use std::fmt;
use std::path::Path;
use std::process::{Command, Stdio};

#[macro_export]
macro_rules! log {
	($($tts:tt)*) => {
		if env::var("GUP_XTRACE").ok().as_deref() == Some("1") {
			println!($($tts)*);
		}
	}
}

pub fn runix_exe() -> &'static str {
	let path = "./runix";
	run_ref(Command::new("gup").arg("-u").arg(path));
	path
}

#[derive(Debug)]
pub struct Buildable {
	pub platform: String,
	pub name: String,
}

impl Buildable {
	pub fn path(&self) -> String {
		format!("platforms/{}/{}", &self.platform, &self.name)
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
		Err(err) => panic!("Spawn failed: {:?} -- ${:?}", &err, &desc),
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

pub fn readlink_str<P: AsRef<Path>>(p: P) -> String {
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

#[derive(Debug)]
#[must_use]
pub struct Dependency {
	pub _path: String,
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
