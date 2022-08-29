use anyhow::*;
use std::{process::Command, path::PathBuf, fs};

fn run_exe(pname: &str, args: Vec<&str>) -> Result<String> {
	let mut path = PathBuf::from("../sample/store-paths");
	path.push(pname);
	assert!(Command::new("gup").arg("-u").arg(&path).spawn()?.wait()?.success());
	let store_path = fs::read_to_string(path)?;

	let mut cmd = Command::new("../target/debug/runix");
	cmd.arg("--cache").arg(store_path.trim()).args(args);
	println!("{:?}", &cmd);
	let output = cmd.output()?;
	println!("{:?}", &output);
	Ok(String::from_utf8(output.stdout)?)
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
	assert_contains("TODO", output)
}
