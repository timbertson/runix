use crate::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Release {
	pub assets_url: String,
	pub upload_url: String,
	pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseAsset {
	pub id: i64,
	pub name: String,
	pub url: String,
}

pub fn curl() -> Command {
	let mut c = Command::new("curl");
	c.arg("--silent").arg("--show-error").arg("--location");
	c.arg("-H").arg(format!("Authorization: Bearer {}", env::var("GITHUB_TOKEN").expect("$GITHUB_TOKEN")));
	c.arg("-H").arg("Accept: application/vnd.github+json");
	c
}

impl Release {
	pub fn fetch(name: &str) -> Self {
		let json = run_output_ref(
			curl().arg(format!("https://api.github.com/repos/timbertson/runix/releases/tags/{}", name)));
		log!("Release JSON: {}", &json);
		serde_json::from_str(&json).expect("parse release JSON")
	}

	pub fn replace_asset(&self, path: &str) {
		let name = path.rsplit('/').next().unwrap();

		// delete any existing asset for this platform
		for asset in self.assets.iter() {
			if asset.name == name {
				log!("Deleting asset {:?}", asset);
				run_ref(curl().arg("-X").arg("DELETE").arg(&asset.url));
			}
		}

		// upload new asset
		run_ref(curl()
			.arg("-X").arg("POST")
			.arg("-H").arg("Content-Type: application/octet-stream")
			.arg("--data-binary").arg(format!("@{}", path))
			.arg(format!("{}?name={}", self.upload_url.split('{').next().unwrap(), name))
			.stdout(Stdio::null())
		);
	}
}
