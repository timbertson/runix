use anyhow::*;
use log::*;
use serde::de::DeserializeOwned;
use std::{process::{Command, Stdio}, collections::HashMap};

// single expr:
//  nix-instantiate --eval --expr '($EXPR).outPath'
// multiplatofrm:
//  nix-instantiate --eval --strict --json --expr 'with builtins; mapAttrs (n: v: v.outPath) ($EXPR)'

fn evaluate_expr<T: DeserializeOwned>(expr: &str) -> Result<T> {
	debug!("Evaluating nix expression: {}", expr);
	let mut cmd = Command::new("nix-instantiate");
	cmd.arg("--eval")
		.arg("--strict")
		.arg("--json")
		.arg("--expr")
		.arg(expr)
		.stderr(Stdio::inherit());
	debug!("{:?}", &cmd);
	let output = cmd.output().with_context(||format!("Running {:?}", &cmd))?;
	if output.status.success() {
		Ok(serde_json::from_slice(&output.stdout)?)
	} else {
		Err(anyhow!("Command failed: {:?}", &cmd))
	}
}

pub fn evaluate_single(expr: &str) -> Result<String> {
	evaluate_expr(&format!("({}).outPath", expr))
}

pub fn evaluate_multi(expr: &str) -> Result<HashMap<String, String>> {
	evaluate_expr(&format!("builtins.mapAttrs (_: d: d.outPath) ({})", expr))
}
