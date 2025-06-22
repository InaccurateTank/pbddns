//! API response objects.

use std::net::IpAddr;
use serde::Deserialize;
use crate::{DnsTypes, helpers};

/// Ping authentication testing endpoint response.
#[derive(Deserialize)]
pub struct Ping {
	/// The IP used for the command.
	#[serde(rename = "yourIp")]
	pub ip: IpAddr
}

/// A single DNS record as returned by the API.
#[derive(Deserialize)]
pub struct DnsRecord {
	/// The identification number of the record.
	#[serde(deserialize_with = "helpers::string_to_u64")]
	pub id: u64,
	/// The full name of the DNS record.
	pub name: String,
	/// The type of the DNS record. See [DnsTypes] for all valid types.
	#[serde(rename = "type")]
	pub record_type: DnsTypes,
	/// The answer content for the record.
	pub content: String,
	/// The current time to live in seconds for the record.
	#[serde(deserialize_with = "helpers::string_to_u64")]
	pub ttl: u64,
	/// The priority of the record.
	#[serde(deserialize_with = "helpers::string_to_optional_u32")]
	pub prio: Option<u32>,
	/// Any notes that have been added to the record on the web panel.
	pub notes: Option<String>,
}

/// DNS record retrieval response.
#[derive(Deserialize)]
pub struct DnsRecordList {
	/// The list of DNS records
	pub records: Vec<DnsRecord>
}

/// SSL certificate bundle.
#[derive(Deserialize, Default)]
pub struct SslBundle {
	/// The complete certificate chain.
	#[serde(rename = "certificatechain")]
	pub certificate_chain: String,
	/// The private key.
	#[serde(rename = "privatekey")]
	pub private_key: String,
	/// The public key.
	#[serde(rename = "publickey")]
	pub public_key: String
}
