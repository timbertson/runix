use anyhow::*;
use runix_build::run_output;
use serial_test::serial;
use std::{process::{Command, Stdio}, path::PathBuf, fs, env};
use itertools::Itertools;

fn runix_cmd() -> Result<Command> {
	let runix_exe = "../build/runix";
	assert!(Command::new("gup").arg("-u").arg(runix_exe).spawn()?.wait()?.success());
	Ok(Command::new(runix_exe))
}

fn run_store_path(pname: &str, args: Vec<&str>) -> Result<String> {
	let mut path = PathBuf::from("../build/store-paths");
	path.push(format!("{}.drv", pname));
	assert!(Command::new("gup").arg("-u").arg(&path).spawn()?.wait()?.success());
	let store_path = fs::read_to_string(path)?;

	let mut cmd = runix_cmd()?;
	cmd.env("RUNIX_CHECK", "1");
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
	let result = cmd.spawn().with_context(||format!("Spawn {:?}", &cmd))?.wait()?;
	if result.success() {
		Ok(())
	} else {
		Err(anyhow!("Comand failed: {:?}", cmd))
	}
}

fn build_wrapper(pname: &str) -> PathBuf {
	let mut path = PathBuf::from("../build/wrappers");
	path.push(pname);
	assert!(Command::new("gup").arg("-u").arg(&path).spawn().unwrap().wait().unwrap().success());
	path
}

fn run_wrapper(pname: &str, args: Vec<&str>) -> Result<String> {
	let path = build_wrapper(pname);
	
	let mut cmd = Command::new(&path);
	cmd.args(args.clone());
	let output = cmd_output(cmd)?;

	// aside: assert that all variants also succeed
	for alt_path in vec!(path.with_extension("multiplatform-attrs"), path.with_extension("multiplatform")) {
		let mut cmd = Command::new(alt_path);
		cmd.args(args.clone());
		cmd.stdout(Stdio::null());
		assert_eq!(Some(0), cmd.status()?.code());
	}
	Ok(output)
}

fn assert_contains<S: AsRef<str>>(needle: &'static str, data: S) -> Result<()> {
	let data_ref = data.as_ref();
	if data_ref.contains(needle) {
		Ok(())
	} else {
		Err(anyhow!("Value doesn't contain '{}':\n\n{}", needle, data_ref))
	}
}

#[test]
#[serial]
fn gnupg() -> Result<()> {
	let output = run_store_path("gnupg", vec!("gpg", "--help"))?;
	assert_contains("gpg (GnuPG)", output)
}

const JQ_HELP_TEXT: &str = "jq - commandline JSON processor";

#[test]
#[serial]
fn jq() -> Result<()> {
	let output = run_wrapper("jq", vec!("--help"))?;
	assert_contains(JQ_HELP_TEXT, output)
}

#[test]
#[serial]
fn validate_jq_multiplatform() -> Result<()> {
	let mut jq = build_wrapper("jq");
	jq.set_extension("multiplatform");
	let mut cmd = runix_cmd()?;
	cmd.arg("--validate").arg(jq);
	assert_eq!(Some(0), cmd.status()?.code());
	Ok(())
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

fn current_platform() -> Result<String> {
	let stdout = |cmd: &mut Command| {
		String::from_utf8(cmd.output().unwrap().stdout).unwrap().trim().to_string()
	};

	Ok(format!("{}-{}",
		stdout(Command::new("uname").arg("-m")),
		stdout(Command::new("uname").arg("-s"))
	))
}

// ###################################################
// expensive integration tests; run with --ignored
// ###################################################

#[test]
#[serial]
#[ignore]
fn local_bootstrap() -> Result<()> {
	let platform_build = format!("../build/platforms/{}", current_platform()?);
	run(Command::new("gup")
		.arg("-u")
		.arg(format!("{}/bootstrap", &platform_build))
	)?;

	test_in_temp_runix(|root| {
		run(Command::new("../bootstrap.sh")
			.env("RUNIX_ROOT", root)
			.env("LOCAL_BOOTSTRAP", platform_build)
		)
	})
}

#[test]
#[serial]
#[ignore]
fn local_auto_bootstrap() -> Result<()> {
	let platform_build = format!("../build/platforms/{}", current_platform()?);
	run(Command::new("gup")
		.arg("-u")
		.arg(format!("{}/bootstrap", &platform_build))
	)?;
	let mut jq_wrapper = build_wrapper("jq");
	jq_wrapper.set_extension("bootstrap");

	test_in_temp_runix(|root| {
		let mut cmd = Command::new(jq_wrapper);
		cmd.arg("--help")
			.env("RUNIX_ROOT", root)
			.env("LOCAL_BOOTSTRAP", platform_build);
		let output = cmd_output(cmd).context("wrapper script")?;
		assert_contains("Note: runix not detected; bootstrapping ...", &output)?;
		assert_contains(JQ_HELP_TEXT, &output)?;
		Ok(())
	})
}


fn all_platforms() -> Vec<&'static str> {
	vec!(
		"x86_64-Linux",
		"x86_64-Darwin",
		"aarch64-Darwin",
	)
}

#[test]
#[ignore]
fn crossplatform_file_types() -> Result<()> {
	let platforms = "../build/platforms";
	run(Command::new("gup")
		.arg("-u")
		.arg(format!("{}/all/bootstrap.dir", &platforms))
	)?;
	for platform in all_platforms() {
		let mut find_cmd = Command::new("find");
		find_cmd.arg(format!("{}/{}/bootstrap.dir", platforms, platform))
			.arg("-type").arg("f")
			.arg("-executable")
			.arg("-exec").arg("file").arg("{}").arg(";");
		for full_line in cmd_output(find_cmd)?.lines() {
			let line = full_line.rsplit(": ").next().unwrap();
			if line.trim().is_empty() {
				continue
			}
			if line.contains("ASCII text") {
				continue
			} else {
				match platform.split_once('-').unwrap() {
					(arch, os) => {
						let arch_name = match arch {
							"x86_64" => "x86-64",
							"aarch64" => "arm64",
							_ => todo!(),
						};
						let exe_type = match os {
							"Darwin" => "Mach-O",
							"Linux" => "ELF",
							_ => todo!(),
						};
						println!("FILE OUTPUT: {}", line);
						assert_contains(arch, line).or(assert_contains(arch_name, line))?;
						assert_contains(exe_type, line)?;
					}
				}
			}
		}
	}
	Ok(())
}

#[test]
#[serial]
#[ignore]
fn linux_bootstrap_in_docker() -> Result<()> {
	// This checks linux on darwin. There's no way to check darwin on linux
	let platform = current_platform()?;
	if platform.starts_with("Linux") {
		println!("Skipping test; already on Linux");
		return Ok(())
	}

	let platform_build = "build/platforms/x86_64-Linux";
	run(Command::new("gup")
		.arg("-u")
		.arg(format!("../{}/bootstrap", &platform_build))
	)?;
	
	let root_dir = env::current_dir()?.parent().unwrap().to_string_lossy().to_string();

	run(Command::new("docker")
		.arg("build")
		.arg("--quiet")
		.arg("--tag").arg("runix-linux-test")
		.arg("--file").arg("Dockerfile.test")
		.arg(".")
		.current_dir(&root_dir)
	)?;

	let mut run_cmd = Command::new("docker");
	run_cmd
		.arg("run")
		.arg("--rm")
		.arg("--volume").arg(format!("{}:/app", &root_dir))
		.arg("--env").arg(format!("LOCAL_BOOTSTRAP=/app/{}", &platform_build))
		.arg("--env").arg("PATH=/bin:/usr/bin:/usr/local/bin:/home/app/bin")
		.arg("runix-linux-test")
		.arg("bash").arg("-euxc")
		.arg("/app/bootstrap.sh && ~/bin/runix --help && ~/bin/runix /app/build/wrappers/jq.multiplatform --help");
	let output = run_output(run_cmd);

	// usage text from runix --help
	assert_contains("runix RUNSCRIPT [...ARGS]", &output)?;

	assert_contains(JQ_HELP_TEXT, &output)?;
	Ok(())
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
