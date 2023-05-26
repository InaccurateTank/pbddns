use std::sync::Arc;
use gumdrop::Options;

mod command;
mod response;
mod config;

pub type Error = Box<dyn std::error::Error>;

pub fn concat(a: &str, b: &str) -> String {
  let mut result: String = String::with_capacity(a.len() + b.len());
  result += a;
  result += b;
  result
}

#[derive(Debug, Options)]
pub struct Opts {
	help: bool,
	#[options(help = "Set data folder location", default = "data/")]
	data: String,
	#[options(help = "Use detailed logging")]
	verbose: bool
}

fn main() -> Result<(), Error> {
	// Opts
	let opts = Opts::parse_args_default_or_exit();

	// Data Folder From Opts
	let data_folder = if !opts.data.ends_with('/') {
		concat(&opts.data, "/")
	} else {
		opts.data
	};

	// Config
	let config = Arc::new(config::Config::new(&data_folder).unwrap());
  let client = reqwest::blocking::Client::new();

	// Fetch current client IP
	let ip: String;
	match client.post("https://porkbun.com/api/json/v3/ping")
		.json(&command::Key{
			apikey: config.apikey.clone(),
			secretapikey: config.secretapikey.clone()
		})
		.send()?
		.json::<response::Ping>()? {
		response::Ping::Success { your_ip } => {
			ip = your_ip;
		},
		response::Ping::Error(e) => {
			return Err(Error::from(e.message))
		}
	};

	for domain in &config.domains {
		// Verbose printing
		if opts.verbose {println!("Now updating domain: \"{}\"", &domain.name)}

		// Array for tracking changes. Stored in order of Changed/Skip/Error
		let mut change: [i8; 3] = [0,0,0];

		// Fetch the list of records from Porkbun and start working on them.
		match client.post(format!("https://porkbun.com/api/json/v3/dns/retrieve/{}", domain.name))
			.json(&command::Key{
				apikey: config.apikey.clone(),
				secretapikey: config.secretapikey.clone()
			})
			.send()?
			.json::<response::Records>()? {
			response::Records::Success { records } => {
				// Filter list of records down to what we actually need
				let list: Vec<response::Record> = records.into_iter()
					.filter(|r| {
						r.name.ends_with(&domain.name) || r.name == domain.name
					}).collect();

				// Iterate through subdomains and find the record for each
				for s in &domain.subdomains {
					match list.iter().find(|x| x.name == format!("{s}.{}", domain.name)) {
						Some(r) => {
							if r.content != ip {
								match client.post(format!("https://porkbun.com/api/json/v3/dns/edit/{}/{}", domain.name, r.id))
									.json(&command::Edit {
										apikey: config.apikey.clone(),
										secretapikey: config.secretapikey.clone(),
										content: ip.clone(),
										ttl: r.ttl.clone().unwrap()
									})
									.send()?
									.json::<response::Edit>()? {
									response::Edit::Success => {
										// Record has been changed
										change[0] += 1;
										if opts.verbose {println!("Record \"{}.{}\" successfully updated", s, &domain.name)}
									},
									response::Edit::Error(e) => {
										// Error if record can't be changed
										change[2] += 1;
										if opts.verbose {println!("Record \"{}.{}\" failed to update: {}", s, &domain.name, e.message)}
									}
								}
							} else {
								// Skip
								change[1] += 1;
								if opts.verbose {println!("Record \"{}.{}\" skipped", s, &domain.name)}
							}
						},
						None => {
							// Error if no record exists
							change[2] += 1;
							if opts.verbose {println!("Record \"{}.{}\" does not exist", s, &domain.name)}
						}
					}
				}
				println!("Updated domain records for \"{}\": {} Changed, {} Skipped, {} Errored", domain.name, change[0], change[1], change[2])
			},
			response::Records::Error(e) => {
				println!("Failed to update domain \"{}\": {}", domain.name, e.message)
			}
		}
	}
  Ok(())
}
