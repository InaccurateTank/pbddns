#![warn(missing_docs)]
//! This crate provibes structs to be used as a framework for accessing the [porkbun](https://porkbun.com) API.

use serde::{Deserialize, Serialize};

pub mod commands;
pub mod error;
pub mod response;
mod uri;
pub use uri::ApiEndpoint;

/// Marker trait that identifies the struct as a valid API command.
pub trait ApiCommand {}

/// Holds the credentials used to access the API.
///
/// For a fair portion of the API this is a valid payload in and of itself. In order to add additional data to the payload use [Keyring::as_long_command()]
#[derive(Deserialize, Serialize)]
pub struct Keyring {
	secretapikey: String,
	apikey: String
}
impl Keyring {
	/// Creates a new [Keyring] from the given API keys.
	pub fn new(
		secret: impl Into<String>,
		key: impl Into<String>
	) -> Self {
		Self {
			secretapikey: secret.into(),
			apikey: key.into()
		}
	}

	/// Creates a [LongCommand] out of a [Keyring] reference and a generic serializable payload.
	pub fn as_long_command<'a, T: ApiCommand>(&'a self, payload: T) -> LongCommand<'a, T> {
		LongCommand {
			keyring: self,
			payload
		}
	}
}

/// Generic container for API commands utilizing a [Keyring].
#[derive(Serialize)]
pub struct LongCommand<'a, T: ApiCommand> {
	#[serde(flatten)]
	keyring: &'a Keyring,
	#[serde(flatten)]
	payload: T
}

/// List of all valid DNS types.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum DnsTypes {
	/// Address Record
	A,
	/// Mail Exchange Record
	MX,
	/// Canonical Name Record
	CNAME,
	/// CNAME Flattening Record
	ALIAS,
	/// Text Record
	TXT,
	/// Name Server Record
	NS,
	/// IPv6 Address Record
	AAAA,
	/// Service Record
	SRV,
	/// TLS Authentication Record
	TLSA,
	/// Certification Authority Authorization
	CAA,
	/// HTTPS Service Record
	HTTPS,
	/// Service Binding Record
	SVCB
}
impl std::fmt::Display for DnsTypes {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::A => write!(f, "A"),
			Self::MX => write!(f, "MX"),
			Self::CNAME => write!(f, "CNAME"),
			Self::ALIAS => write!(f, "ALIAS"),
			Self::TXT => write!(f, "TXT"),
			Self::NS => write!(f, "NS"),
			Self::AAAA => write!(f, "AAAA"),
			Self::SRV => write!(f, "SRV"),
			Self::TLSA => write!(f, "TLSA"),
			Self::CAA => write!(f, "CAA"),
			Self::HTTPS => write!(f, "HTTPS"),
			Self::SVCB => write!(f, "SVCB"),
		}
	}
}
