use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
	#[error("Could not read the configuration file {path}")]
	MissingFile {
		path: Box<std::path::Path>,
		source: std::io::Error
	}
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("One or more subdomains are missing from Porkbun: {0:?}")]
	MissingSubdomains(Vec<String>),
	#[error("Record is not an A/AAAA record and is thus unsafe to edit: {0}")]
	UnsafeRecord(String)
}
