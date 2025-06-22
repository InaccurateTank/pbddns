use std::{collections::HashMap, net::IpAddr};
use color_eyre::{Result, Section};
use pb_api::{commands, responses};
use tracing::{debug, error, info, instrument};

use crate::structs::config;

mod error;
mod structs;

// type CmdGenerator = Box<dyn Fn(&Option<String>) -> commands::CreateOrEditRecord>;

#[instrument]
fn main() -> Result<()> {
	color_eyre::install()?;

	// Opts
	let opts = structs::Opts::parse();

	// Install tracing subscriber
	install_tracing(&opts);

	// Final preperations
	let config = structs::config::Config::load(opts.config)?;

	// Construct an agent using a config
	let mut agent_config = ureq::Agent::config_builder()
		.timeout_global(Some(std::time::Duration::from_secs(5)))
		.https_only(true);
	if opts.forcev4 {
		agent_config = agent_config.ip_family(ureq::config::IpFamily::Ipv4Only)
	}
	let agent = agent_config.build()
		.new_agent();

	// Create a API framework
	let framework = pb_api::Framework::new(config.endpoint, &agent, config.keyring);

	// Fetch ip from API for comparison.
	let ip = framework.ping()?.ip;

	// Status tracker
	// let mut tracker = structs::CommandStatus::default();
	// Actual DDNS work
	for domain in config.domains {
		// SSL Fetch
		if let Some(ssl) = &domain.ssl {
			// Might not exist so is skippable
			if let Err(e) = retrieve_ssl(&domain.name, &ssl, &framework) {
				error!("{e}");
			}
		}
		// DDNS
		if domain.update_tld || !domain.subdomains.is_empty() {
			if let Err(e) = update_domain_records(ip, domain, &framework) {
				error!("{e}");
			}
			// update_domain_records(ip, domain, &framework)?;
		}
		// let records: HashMap<Option<String>, responses::DnsRecord> = framework.records_by_domain_or_id(&domain.name, None)?
		// 	.records
		// 	.into_iter()
		// 	.filter_map(|f| {
		// 		if domain.update_tld && f.name == domain.name {
		// 			return Some((None, f))
		// 		}
		// 		if let Some(index) = domain.subdomains.iter().position(|x| f.name.starts_with(x)) {
		// 			return Some((Some(domain.subdomains.swap_remove(index)), f))
		// 		}
		// 		None
		// 	})
		// 	.collect();

		// // If the subdomain list is not empty then a record is missing. Best to just error this.
		// if !domain.subdomains.is_empty() {
		// 	return Err(error::Error::MissingSubdomains(domain.subdomains))
		// 		.suggestion("Double check the spelling and existance of the subdomain.")
		// }

		// // Iterate through subdomains
		// for (subdomain, record) in records {
		// 	// Check record type before attempting anything
		// 	if record.record_type != pb_api::DnsTypes::A &&
		// 		record.record_type != pb_api::DnsTypes::AAAA
		// 	{
		// 		let e = error::Error::UnsafeRecord(record.name);
		// 		tracker.add_errored();
		// 		error!("{e}");
		// 		continue;
		// 	}
		// 	// Create edit command
		// 	let cmd = record_command(&subdomain)
		// 		.with_priority(record.prio)
		// 		.with_ttl(record.ttl);
		// 	// Compare the source and destination ips
		// 	if cmd.content == record.content {
		// 		tracker.add_skipped();
		// 		debug!("Skipping record - Identical IPs: {}", record.name);
		// 		continue;
		// 	}
		// 	// Error or not?
		// 	if let Err(e) = framework.edit_by_domain_and_id(&domain.name, record.id, cmd) {
		// 		tracker.add_errored();
		// 		error!("{e}");
		// 	} else {
		// 		debug!("Record changed: {}", record.name);
		// 		tracker.add_changed();
		// 	}
		// }
	}
	// info!("All records updated: {tracker}");
  Ok(())
}

fn install_tracing(
	options: &structs::Opts
) {
	use tracing::Level;
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;

	let verbosity = match options.verbose {
		0 => Level::INFO,
		1 => Level::DEBUG,
		_ => Level::TRACE
	};
	let fmt_layer = tracing_subscriber::fmt::layer()
		.with_writer(std::io::stdout.with_max_level(verbosity))
		.without_time()
		.with_target(false);

	tracing_subscriber::registry()
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}

#[instrument(skip(ssl, framework))]
fn retrieve_ssl(
	domain: &str,
	ssl: &config::SslConfig,
	framework: &pb_api::Framework
) -> Result<()> {
	use std::fs::write;
	let bundle = framework.retrieve_ssl_by_domain(domain)
		.map_err(|e| error::Error::SslFetchError { domain: domain.into(), source: e })?;

	fn write_key(
		path: &std::path::Path,
		key: String
	) {
		match write(path, key)
			.map_err(|e| error::Error::SslWriteError { path: path.to_path_buf(), source: e })
		{
			Ok(_) => debug!("Successfully wrote file {}.", path.to_string_lossy()),
			Err(e) => error!("{e}")
		}
	}

	if let Some(chain_path) = &ssl.chain {
		write_key(chain_path, bundle.certificate_chain);
	}
	if let Some(private_path) = &ssl.private {
		write_key(private_path, bundle.private_key);
	}
	if let Some(public_path) = &ssl.public {
		write_key(public_path, bundle.public_key);
	}
	info!("All ssl files retrieved for {domain}.");
	Ok(())
}

#[instrument(skip_all, fields(domain=%domain.name))]
fn update_domain_records(
	ip: IpAddr,
	domain: structs::config::DomainConfig,
	framework: &pb_api::Framework
	// tracker: &mut structs::CommandStatus
) -> Result<()> {
	// Only now instantiate mutability
	let mut subdomains = domain.subdomains;
	let mut tracker = structs::CommandStatus::default();
	// Create records list
	let records: HashMap<Option<String>, responses::DnsRecord> = framework.records_by_domain_or_id(&domain.name, None)
		.map_err(|e| error::Error::RecordFetchError { domain: domain.name.to_owned(), source: e })?
		.records
		.into_iter()
		.filter_map(|f| {
			if domain.update_tld && f.name == domain.name {
				return Some((None, f))
			}
			if let Some(index) = subdomains.iter().position(|x| f.name.starts_with(x)) {
				return Some((Some(subdomains.swap_remove(index)), f))
			}
			None
		})
		.collect();

	// If the subdomain list is not empty then a record is missing. Best to just error this.
	if !subdomains.is_empty() {
		return Err(error::Error::MissingSubdomains(subdomains))
			.suggestion("Double check the spelling and existance of the reported subdomains.")
	}

	// Iterate through subdomains
	for (subdomain, record) in records {
		// Create edit command if the record type is A or AAAA
		let cmd = match record.record_type {
			pb_api::DnsTypes::A |
			pb_api::DnsTypes::AAAA => {
				commands::CreateOrEditRecord::new_a_or_aaaa(subdomain.as_deref(), ip)
					.with_priority(record.prio)
					.with_ttl(record.ttl)
			},
			_ => {
				let e: error::Error = error::Error::UnsafeRecord(record.name);
				tracker.add_errored();
				error!("{e}");
				continue;
			}
		};
		// Compare the source and destination ips
		if cmd.content == record.content {
			tracker.add_skipped();
			debug!("Skipping record - Identical IPs: {}", record.name);
			continue;
		}
		// Error or not?
		if let Err(e) = framework.edit_by_domain_and_id(&domain.name, record.id, cmd) {
			tracker.add_errored();
			error!("{e}");
		} else {
			debug!("Record changed: {}", record.name);
			tracker.add_changed();
		}
	}
	info!("All records for {} updated: {}", domain.name, tracker);
	Ok(())
}
