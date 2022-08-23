use anyhow::*;
use log::*;
use std::{process::Command, os::unix::process::CommandExt};

pub fn main() -> Result<()> {
	env_logger::init();
	let inject = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/librunix_inject.dylib");
	debug!("Injecting: {:?}", inject);
	Err(Command::new("id")
		.arg("-u")
		.env("DYLD_INSERT_LIBRARIES", inject)
		.exec().into())
}
