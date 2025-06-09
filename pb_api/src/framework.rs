use serde::{Deserialize, Serialize};
use tracing::instrument;
use ureq::http::{StatusCode, Uri};
use crate::{commands, error, responses, ApiEndpoint, ApiResponse, Keyring};

/// Utilizes an [Agent][ureq::Agent], an [ApiEndpoint], and a [Keyring] in order to make safe requests to the Porkbun API.
///
/// Note that the framework steals the command names from [ApiEndpoint].
pub struct Framework<'a> {
	client: &'a ureq::Agent,
	endpoint: ApiEndpoint<'a>,
	keyring: Keyring
}
impl<'a> Framework<'a> {
	/// Creates a new API framework using an [Agent][ureq::Agent], an [ApiEndpoint], and a [Keyring]
	///
	/// Consumes both the [ApiEndpoint] and the [Keyring] but only takes the [Agent][ureq::Agent] by reference.
	pub fn new(endpoint: ApiEndpoint<'a>, client: &'a ureq::Agent, keyring: Keyring) -> Self {
		Self {
			client,
			endpoint,
			keyring
		}
	}

	#[instrument(skip(self, cmd))]
	fn post<C: Serialize, R: for<'de> Deserialize<'de>>(
		&self,
		endpoint: Uri,
		cmd: C,
	) -> Result<R, error::Error> {
		let (head, mut body) = self.client.post(endpoint)
			.send_json(cmd)?
			.into_parts();
		let result = body.read_json::<ApiResponse<_>>()?;
		match (head.status, result) {
			(StatusCode::OK, ApiResponse::Success(x)) => Ok(x),
			(status, message) => {
				let error = match message {
					ApiResponse::Success(_) => None,
					ApiResponse::Error(e) => Some(e)
				};
				Err(
					error::ApiError {
						status,
						error
					}.into()
				)
			}
		}
	}

	#[instrument(skip(self))]
	/// Endpoint used to test the credentials within a [Keyring][crate::Keyring].
	pub fn ping(
		&self
	) -> Result<responses::Ping, error::Error> {
		self.post(self.endpoint.ping(), &self.keyring)
	}

	#[instrument(skip(self))]
	/// Retrieve all editable DNS records associated with a domain or a single record for a particular record ID.
	pub fn records_by_domain_or_id(
		&self,
		domain: impl AsRef<str> + std::fmt::Debug,
		id: Option<&'a u64>
	) -> Result<responses::DnsRecordList, error::Error> {
		self.post(self.endpoint.records_by_domain_or_id(domain, id), &self.keyring)
	}

	#[instrument(skip(self, cmd))]
	/// Edits a DNS record based on the supplied domain and ID
	pub fn edit_by_domain_and_id(
		&self,
		domain: impl AsRef<str> + std::fmt::Debug,
		id: u64,
		cmd: commands::CreateOrEditRecord
	) -> Result<(), error::Error> {
		self.post(self.endpoint.edit_by_domain_and_id(domain, id), self.keyring.with(cmd))
	}
}
