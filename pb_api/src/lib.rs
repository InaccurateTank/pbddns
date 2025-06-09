#![warn(missing_docs)]
//! This crate provibes structs to be used as a framework for accessing the [porkbun](https://porkbun.com) API.

use serde::{Deserialize, Serialize};

pub mod commands;
mod endpoint;
pub use endpoint::ApiEndpoint;
pub mod error;
mod framework;
pub use framework::Framework;
mod helpers;
pub mod responses;

/// Marker trait that identifies the struct as a valid API command.
pub trait ApiCommand {}

/// Holds the credentials used to access the API.
///
/// For a fair portion of the API this is a valid payload in and of itself. In order to add additional data to the payload use [Keyring::with()]
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

	/// Creates a [WithKeyring] to use as the the payload.
	pub fn with<'a, C: ApiCommand + Serialize>(
		&'a self,
		inner: C
	) -> WithKeyring<'a, C> {
		WithKeyring {
			keyring: self,
			inner
		}
	}
}

/// Generic container for API commands utilizing a [Keyring].
#[derive(Serialize)]
pub struct WithKeyring<'a, C: ApiCommand> {
	#[serde(flatten)]
	keyring: &'a Keyring,
	#[serde(flatten)]
	inner: C
}

/// Contains the recieved error message from the API.
#[derive(Debug, Deserialize)]
pub struct ApiErrorMessage {
	/// The error message.
	pub message: String
}
impl std::fmt::Display for ApiErrorMessage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.message.fmt(f)
	}
}

/// Generic enum for responses from the API.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
#[serde(tag = "status", rename_all = "UPPERCASE")]
pub enum ApiResponse<T> {
	Success(T),
	Error(ApiErrorMessage)
}
impl<T> From<ApiResponse<T>> for std::result::Result<T, ApiErrorMessage> {
	fn from(value: ApiResponse<T>) -> Self {
		match value {
			ApiResponse::Success(s) => Ok(s),
			ApiResponse::Error(e) => Err(e),
		}
	}
}

/// List of all valid DNS types.
#[derive(Deserialize, Serialize, PartialEq, Eq)]
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
