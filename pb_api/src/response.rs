//! API response objects.

use serde::Deserialize;
use crate::DnsTypes;

/// Contains the recieved error message from the API.
#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
	/// The error message.
	pub message: String
}

/// Generic enum for responses from the API.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
#[serde(tag = "status", rename_all = "UPPERCASE")]
pub enum ApiResponse<T> {
	Success(T),
	Error(ErrorMessage)
}

/// A single DNS record as returned by the API.
#[derive(Debug, Deserialize)]
pub struct DnsRecord {
	/// The identification number of the record.
	pub id: u64,
	/// The subdomain name of the DNS record.
	pub name: String,
	/// The type of the DNS record. See [DnsTypes] for all valid types.
	#[serde(rename = "type")]
	pub record_type: DnsTypes,
	/// The answer content for the record.
	pub content: String,
	/// The current time to live in seconds for the record.
	pub ttl: u64,
	/// The priority of the record.
	pub prio: u64,
	/// Any notes that have been added to the record on the web panel.
	pub notes: Option<String>,
}
