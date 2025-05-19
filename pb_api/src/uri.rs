use std::borrow::Cow;

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
	/// Returns the IP address used to make the request.
	pub fn ping(&self) -> String {
		format!("{}/ping", self.0)
	}
}
impl<'a> std::fmt::Display for ApiEndpoint<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
