use anyhow::*;
use std::{process::Command, path::PathBuf, fs};
use itertools::Itertools;

fn run_exe(pname: &str, args: Vec<&str>) -> Result<String> {
	let mut path = PathBuf::from("../build/store-paths");
	path.push(format!("{}.drv", pname));
	assert!(Command::new("gup").arg("-u").arg(&path).spawn()?.wait()?.success());
	let store_path = fs::read_to_string(path)?;

	let mut cmd = Command::new("../target/debug/runix");
	cmd.arg("--require").arg(store_path.trim()).args(args);
	cmd_output(cmd)
}

fn cmd_output(mut cmd: Command) -> Result<String> {
	println!("{:?}", &cmd);
	let output = cmd.output()?;
	let stderr = String::from_utf8(output.stderr)?;
	let stdout = String::from_utf8(output.stdout)?;
	let all_lines =
		stderr.lines().map(|l| format!("[stdout] {}", l)).chain(
			stdout.lines().map(|l| format!("[stderr] {}", l))
		);
	#[allow(unstable_name_collisions)]
	Ok(all_lines.intersperse("\n".to_owned()).collect())
}

fn run_wrapper(pname: &str, args: Vec<&str>) -> Result<String> {
	let mut path = PathBuf::from("../build/wrappers");
	path.push(pname);
	assert!(Command::new("gup").arg("-u").arg(&path).spawn()?.wait()?.success());
	let mut cmd = Command::new(path);
	cmd.args(args);
	cmd_output(cmd)
}

fn assert_contains(needle: &'static str, data: String) -> Result<()> {
	if data.contains(needle) {
		Ok(())
	} else {
		Err(anyhow!("Value doesn't contain '{}':\n\n{}", needle, data))
	}
}

#[test]
fn gnupg() -> Result<()> {
	let output = run_exe("gnupg", vec!("gpg", "--help"))?;
	assert_contains("gpg (GnuPG) 2.3.6", output)
}

#[test]
fn jq() -> Result<()> {
	let output = run_wrapper("jq", vec!("--help"))?;
	assert_contains("jq - commandline JSON processor [version 1.6]", output)
}
