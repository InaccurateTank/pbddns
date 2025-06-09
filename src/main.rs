use std::{collections::HashMap, net::IpAddr};
use color_eyre::{Result, Section};
use pb_api::{commands, responses};
use tracing::{debug, error, info, instrument};

mod error;
mod structs;

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
	let mut tracker = structs::CommandStatus::new();
	// Closure for command creation based on ip type
	// The API has the magic ability to swap an entries record type. So lets do that!
	let record_command: Box<dyn Fn(&Option<String>) -> commands::CreateOrEditRecord> = match ip {
		IpAddr::V4(ip) => Box::new(move |sub| commands::CreateOrEditRecord::new_a(sub.as_deref(), ip)),
		IpAddr::V6(ip) => Box::new(move |sub| commands::CreateOrEditRecord::new_aaaa(sub.as_deref(), ip))
	};
	// Actual DDNS work
	for (tld, mut domain_config) in config.domains {
		let records: HashMap<Option<String>, responses::DnsRecord> = framework.records_by_domain_or_id(&tld, None)?
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

		// Iterate through subdomains
		for (subdomain, record) in records {
			// Check record type before attempting anything
			if record.record_type != pb_api::DnsTypes::A &&
				record.record_type != pb_api::DnsTypes::AAAA
			{
				let e = error::Error::UnsafeRecord(record.name);
				tracker.add_errored();
				error!("{e}");
				continue;
			}
			// Create edit command
			let cmd = record_command(&subdomain)
				.with_priority(record.prio)
				.with_ttl(record.ttl);
			// Compare the source and destination ips
			if cmd.content == record.content {
				tracker.add_skipped();
				debug!("Skipping record - Identical IPs: {}", record.name);
				continue;
			}
			// Error or not?
			if let Err(e) = framework.edit_by_domain_and_id(&tld, record.id, cmd) {
				tracker.add_errored();
				error!("{e}");
			} else {
				debug!("Record changed: {}", record.name);
				tracker.add_changed();
			}
		}
	}
	info!("All records updated: {tracker}");
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
