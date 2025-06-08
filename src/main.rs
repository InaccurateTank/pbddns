use std::{collections::HashMap, net::IpAddr};
use color_eyre::{Result, Section};
use pb_api::{commands, responses};

mod error;
mod structs;

fn main() -> Result<()> {
	color_eyre::install()?;

	// Opts
	let opts = structs::Opts::parse();

	// Final preperations
	let config = structs::config::Config::load(opts.config)?;

	// Construct an agent
	let agent = ureq::Agent::config_builder()
		.timeout_global(Some(std::time::Duration::from_secs(5)))
		.http_status_as_error(false)
		.build()
		.new_agent();

	// Fetch ip from API for comparison.
	let ip = config.keyring.post::<responses::Ping>(
		&agent,
		config.endpoint.ping()
	)?.ip;

	// Status tracker
	let mut tracker = structs::CommandStatus::new();
	// Closure for command creation based on ip type
	// The API has the magic ability to swap an entries record type. So lets do that!
	let record_command: Box<dyn Fn(&Option<String>) -> commands::CreateOrEditRecord> = match ip {
		IpAddr::V4(ip) => Box::new(move |sub| commands::CreateOrEditRecord::new_a(sub.as_deref(), ip)),
		IpAddr::V6(ip) => Box::new(move |sub| commands::CreateOrEditRecord::new_aaaa(sub.as_deref(), ip))
	};
	// Actual DDNS work
	for (tld, mut domain_config) in config.domains {
		let records: HashMap<Option<String>, responses::DnsRecord> = config.keyring.post::<responses::DnsRecordList>(
			&agent,
			config.endpoint.records_by_domain_or_id(&tld, None)
		)?
			.records
			.into_iter()
			.filter_map(|f| {
				if domain_config.update_tld && f.name == tld {
					return Some((None, f))
				}
				if let Some(index) = domain_config.subdomains.iter().position(|x| f.name.starts_with(x)) {
					return Some((Some(domain_config.subdomains.swap_remove(index)), f))
				}
				None
			})
			.collect();

		// If the subdomain list is not empty then a record is missing. Best to just error this.
		if !domain_config.subdomains.is_empty() {
			return Err(error::Error::MissingSubdomains(domain_config.subdomains))
				.suggestion("Double check the spelling and existance of the subdomain.")
		}

		for (subdomain, record) in records {
			if record.record_type != pb_api::DnsTypes::A && record.record_type != pb_api::DnsTypes::AAAA {
				tracker.add_errored();
				println!("Entry {} is not an A/AAAA record. Unsafe to change.", record.name);
				continue;
			}
			let cmd = record_command(&subdomain)
				.with_priority(record.prio)
				.with_ttl(record.ttl);
			if cmd.content == record.content {
				tracker.add_skipped();
				println!("Skipping record {}: Identical IPs", record.name);
				continue;
			}
			if let Err(e) = config.keyring.post_with::<_, ()>(cmd, &agent, config.endpoint.edit_by_domain_and_id(&tld, record.id)) {
				tracker.add_errored();
				println!("{}", e);
			} else {
				tracker.add_changed();
			}
		}
	}
	println!("All records updated: {tracker}");
  Ok(())
}
