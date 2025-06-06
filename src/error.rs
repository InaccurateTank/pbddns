use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
	#[error("Could not read the configuration file {path}")]
	MissingFile {
		path: Box<std::path::Path>,
		source: std::io::Error
	}
}
