use std::path::PathBuf;

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
	UnsafeRecord(String),
	#[error("Failed to fetch SSL bundle for {domain}: {source}\n      Does the SSL bundle exist?")]
	SslFetchError {
		domain: String,
		source: pb_api::error::Error
	},
	#[error("Failed to write file {}: {source}\n      Check permissions and parent directories.", path.to_string_lossy())]
	SslWriteError{
		path: PathBuf,
		source: std::io::Error
	},
	#[error("Failed to fetch records for tld {domain}: {source}\n      Does the domain exist?")]
	RecordFetchError {
		domain: String,
		source: pb_api::error::Error
	}
}
