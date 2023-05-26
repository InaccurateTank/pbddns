use std::{
	fs::OpenOptions,
	io::{Read, Write}
};
use serde::{Serialize, Deserialize};
use crate::{concat, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub apikey: String,
	pub secretapikey: String,
	pub domains: Vec<Domain>
}
impl Config {
	pub fn new(folder: &str) -> Result<Self, Error> {
		let mut file = OpenOptions::new()
			.create(true)
			.read(true)
			.write(true)
			.open(concat(folder, "config.toml"))?;
		let mut toml = String::new();
		file.read_to_string(&mut toml)?;
		if toml.is_empty() {
			write!(file, r##"apikey = ""
secretapikey = ""

[[domains]]
name = "example.com"
update_tld = false
subdomains = []"##)?;
			Err(String::from("Config file does not exist. Please fill out generated config file before running again.").into())
		} else {
			Ok(toml::from_str(&toml)?)
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
	pub name: String,
	pub update_tld: bool,
	pub subdomains: Vec<String>
}
