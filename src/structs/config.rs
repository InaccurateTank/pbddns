use std::{collections, fs, io};
use color_eyre::{Result, Section};
use serde::Deserialize;
use tracing::{debug, instrument};
use crate::error::ConfigError;

const DEFAULT_CONFIG: &str = r##"endpoint = "https://api.porkbun.com/api/json/v3"
apikey = ""
secretapikey = ""

[[domains]]
name = "example.com"
update_tld = false
subdomains = []"##;

#[derive(Deserialize)]
pub struct Config<'a> {
	pub endpoint: pb_api::ApiEndpoint<'a>,
	#[serde(flatten)]
	pub keyring: pb_api::Keyring,
	pub domains: collections::HashMap<String, DomainConfig>
}
impl Config<'_> {
	#[instrument]
	pub fn load(
		path: impl AsRef<std::path::Path> + std::fmt::Debug
	) -> Result<Self> {
		let config_path = path.as_ref();
		match fs::read_to_string(config_path) {
			Ok(file) => {
				debug!("Successfully loaded configuration from file.");
				Ok(toml::from_str(&file)?)
			},
			Err(e) => {
				let generated: Result<bool, io::Error> = if e.kind() == io::ErrorKind::NotFound {
					if let Some(folder) = config_path.parent() {
						fs::create_dir_all(folder)?;
						debug!("Created config file parent directories.");
					}
					fs::write(config_path, DEFAULT_CONFIG)?;
					debug!("Generated default configuration file.");
					Ok(true)
				} else {
					Ok(false)
				};

				Err(ConfigError::MissingFile {
					path: config_path.into(),
					source: e
				}).with_suggestion(|| {
					if generated.is_ok_and(|x|x) {
						"Please fill out the generated config file before running."
					} else {
						"Please make sure the configuration path is either correct or writable before running."
					}
				})
			}
		}
	}
}

#[derive(Deserialize)]
pub struct DomainConfig {
	pub update_tld: bool,
	pub subdomains: Vec<String>
}
