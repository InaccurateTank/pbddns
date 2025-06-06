//! API response objects.

use std::net::IpAddr;
use serde::Deserialize;
use crate::{DnsTypes, PbResponse, helpers};

/// Ping authentication testing endpoint response.
#[derive(Debug, Deserialize)]
pub struct Ping {
	/// The IP used for the command.
	#[serde(rename = "yourIp")]
	pub ip: IpAddr
}
impl PbResponse for Ping {}

/// A single DNS record as returned by the API.
#[derive(Debug, Deserialize)]
pub struct DnsRecord {
	/// The identification number of the record.
	#[serde(deserialize_with = "helpers::string_to_u64")]
	pub id: u64,
	/// The subdomain name of the DNS record.
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
	#[serde(deserialize_with = "helpers::string_to_u32")]
	pub prio: u32,
	/// Any notes that have been added to the record on the web panel.
	pub notes: Option<String>,
}
impl PbResponse for DnsRecord {}

/// DNS record retrieval response.
#[derive(Debug, Deserialize)]
pub struct DnsRecordList {
	/// The list of DNS records
	pub records: Vec<DnsRecord>
}
impl PbResponse for DnsRecordList {}
