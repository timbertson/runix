use anyhow::*;
use std::{process::Command, path::PathBuf, fs};
use itertools::Itertools;

fn run_exe(pname: &str, args: Vec<&str>) -> Result<String> {
	let mut path = PathBuf::from("../sample/store-paths");
	path.push(pname);
	assert!(Command::new("gup").arg("-u").arg(&path).spawn()?.wait()?.success());
	let store_path = fs::read_to_string(path)?;

	let mut cmd = Command::new("../target/debug/runix");
	cmd.arg("--cache").arg(store_path.trim().strip_prefix("/nix/store/").unwrap()).args(args);
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
