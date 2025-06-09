use std::path::{PathBuf, MAIN_SEPARATOR_STR};
use gumdrop::Options;

#[cfg(unix)]
fn to_pathbuf(s: &str) -> PathBuf {
	PathBuf::from(s)
}
#[cfg(windows)]
fn to_pathbuf(s: &str) -> PathBuf {
	PathBuf::from(s.replace("/", MAIN_SEPARATOR_STR))
}

#[derive(Debug, Options)]
pub struct Opts {
	pub help: bool,
	#[options(help = "The path to the configuration file.", default = "data/config.toml", parse(from_str = "to_pathbuf"))]
	pub config: PathBuf,
	#[options(no_long, count, help = "Increase logging verbosity to DEBUG. Repeat once for TRACE data.")]
	pub verbose: u8,
	#[options(short = "4", no_long, help = "Whether to force the usage of IPv4.")]
	pub forcev4: bool
}

impl Opts {
	/// Thin wrapper around [parse_args_default_or_exit][gumdrop::Options::parse_args_default_or_exit()]
	pub fn parse() -> Self {
		Opts::parse_args_default_or_exit()
	}
}
