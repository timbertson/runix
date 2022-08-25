use anyhow::*;
use log::*;
use std::{process::Command, os::unix::process::CommandExt};

pub fn main() -> Result<()> {
	env_logger::init();
	let inject = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/librunix_inject.dylib");
	debug!("Injecting: {:?}", inject);
	let mut all_args = std::env::args();
	debug!("{:?}", all_args);
	let mut args = all_args.into_iter().skip(1);
	let mut cmd = Command::new(args.next().unwrap());
	cmd.args(args);
	info!("{:?}", cmd);

	Err(cmd
		.env("DYLD_INSERT_LIBRARIES", inject)
		.env("DYLD_FORCE_FLAT_NAMESPACE", "1")
		.exec().into())
}
