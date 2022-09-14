use anyhow::*;
use serial_test::serial;
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

fn run(cmd: &mut Command) -> Result<()> {
	println!("{:?}", &cmd);
	let result = cmd.spawn()?.wait()?;
	if result.success() {
		Ok(())
	} else {
		Err(anyhow!("Comand failed: {:?}", cmd))
	}
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

fn cleanup_runix_root(tmp: &str) -> Result<()> {
	if PathBuf::from(tmp).exists() {
		run(Command::new("chmod").arg("-R").arg("+w").arg(tmp))?;
		run(Command::new("rm").arg("-rf").arg(tmp))
	} else {
		Ok(())
	}
}

fn test_in_temp_runix<F: FnOnce(&str) -> Result<()>>(f: F) -> Result<()> {
	let tmp = "/tmp/runix-tests";
	cleanup_runix_root(tmp)?;
	let result = f(tmp).and_then(|_| {
		let mut cmd = Command::new(format!("{}/current/bin/runix", tmp));
		cmd.arg("--help");
		cmd.env("RUNIX_ROOT", tmp);
		cmd_output(cmd)
	}).and_then(|output| assert_contains("runix RUNSCRIPT [...ARGS]", output));
	cleanup_runix_root(tmp)?;
	result
}

// expensive integration tests; run with --ignored
#[test]
#[serial]
#[ignore]
fn local_bootstrap() -> Result<()> {
	test_in_temp_runix(|root| {
		let mut platform_cmd = Command::new("uname");
		platform_cmd.arg("-m").arg("-s");
		let platform = String::from_utf8(platform_cmd.output()?.stdout)?.trim().replace(' ', "-");
		let platform_build = format!("build/platforms/{}", platform);
		run(Command::new("gup")
			.arg("-u")
			.arg("../build/platforms/current/bootstrap")
		)?;
		run(Command::new("../bootstrap.sh")
			.env("RUNIX_ROOT", root)
			.env("LOCAL_BOOTSTRAP", format!("../{}", &platform_build))
		)
	})
}

#[test]
#[serial]
#[ignore]
fn remote_bootstrap() -> Result<()> {
	test_in_temp_runix(|root| {
		run(Command::new("bash")
				.arg("-euxc")
				.arg("curl -sSL https://raw.githubusercontent.com/timbertson/runix/main/bootstrap.sh | bash")
				.env("RUNIX_ROOT", root)
		)
	})
}
