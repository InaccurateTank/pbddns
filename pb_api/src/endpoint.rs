use std::borrow::Cow;
use ureq::http::Uri;
use serde::Deserialize;

/// In order to keep the endpoint list resistent to future changes it will be calculated from a newtype.
///
/// The root URL is stored and used to construct the various API endpoints via methods.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct ApiEndpoint<'a>(Cow<'a, str>);
impl<'a> ApiEndpoint<'a> {
	/// Create a new instance of [ApiEndpoint] from a string.
	pub fn new(root: impl Into<Cow<'a, str>> ) -> Self {
		Self(root.into())
	}

	/// Endpoint used to test the credentials within a [Keyring][crate::Keyring].
	///
	/// Takes a [Keyring][crate::Keyring] and returns a [Ping][crate::responses::Ping].
	pub fn ping(&self) -> Uri {
		// Frankly if this fails then somthing is deeply wrong
		format!("{}/ping", self.0).parse().unwrap()
	}

	/// Retrieve all editable DNS records associated with a domain or a single record for a particular record ID.
	///
	/// Takes a [Keyring][crate::Keyring] and returns a [DnsRecordList][crate::responses::DnsRecordList].
	pub fn records_by_domain_or_id(
		&self,
		domain: impl AsRef<str>,
		id: Option<&'a str>
	) -> Uri {
		let mut result = format!("{}/dns/retrieve/{}", self.0, domain.as_ref());
		if let Some(id) = id {
			result = format!("{}/{}", result,  id);
		}
		// Frankly if this fails then somthing is deeply wrong
		result.parse().unwrap()
	}
}
impl<'a> std::fmt::Display for ApiEndpoint<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
