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

#[derive(Debug)]
pub enum Status {
	Change,
	Skip,
	Error
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
	let config = config::Config::new(&data_folder).unwrap();
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
						r.name.ends_with(&domain.name) && r.rec_type == "A"
					}).collect();

				let a: String = "Test".to_string();
				a.ends_with("Test");

				// Update TLD if set to do so
				if domain.update_tld {
					match list.iter().find(|x| x.name == domain.name) {
						Some(r) => {
							// Run update function
							match update(&domain.name, r, &ip, &client, &config, opts.verbose)? {
								Status::Change => {
									change[0] += 1;
								},
								Status::Skip => {
									change[1] += 1;
								},
								Status::Error => {
									change[2] += 1;
								}
							}
						},
						None => {
							// Error if no record exists
							change[2] += 1;
							if opts.verbose {println!("Record \"{}\" does not exist", &domain.name)}
						}
					}
				}

				// Iterate through subdomains and find the record for each
				for s in &domain.subdomains {
					let full_name = format!("{s}.{}", domain.name);
					match list.iter().find(|x| x.name == full_name) {
						Some(r) => {
							// Run update function
							match update(&full_name, r, &ip, &client, &config, opts.verbose)? {
								Status::Change => {
									change[0] += 1;
								},
								Status::Skip => {
									change[1] += 1;
								},
								Status::Error => {
									change[2] += 1;
								}
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

fn update(domain: &str, record: &response::Record, ip: &str, client: &reqwest::blocking::Client, config: &config::Config, verbose: bool) -> Result<Status, Error> {
	// Record, Client, Config, Opts
	if record.content != ip {
		match client.post(format!("https://porkbun.com/api/json/v3/dns/edit/{}/{}", domain, record.id))
			.json(&command::Edit {
				apikey: config.apikey.clone(),
				secretapikey: config.secretapikey.clone(),
				content: ip.to_string(),
				ttl: record.ttl.clone().unwrap_or("600".to_string())
			})
			.send()?
			.json::<response::Edit>()? {
			response::Edit::Success => {
				// Record has been changed
				if verbose {println!("Record \"{domain}\" successfully updated")}
				Ok(Status::Change)
			},
			response::Edit::Error(e) => {
				// Error if record can't be changed
				if verbose {println!("Record \"{}\" failed to update: {}", domain, e.message)}
				Ok(Status::Error)
			}
		}
	} else {
		// Skip
		if verbose {println!("Record \"{domain}\" skipped")}
		Ok(Status::Skip)
	}
}
