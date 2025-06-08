//! Errors and error handling QOL.

use thiserror::Error;
use crate::ApiErrorMessage;

/// Actual error to return when [ApiResponse][crate::ApiResponse] returns an error.
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub struct ApiError {
	pub status: ureq::http::StatusCode,
	pub error: Option<ApiErrorMessage>
}
impl std::fmt::Display for ApiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(error) = &self.error {
			write!(f, "{}: {}", self.status, error)
		} else {
			write!(f, "{}", self.status)
		}
	}
}

/// Error returned from library.
#[derive(Error, Debug)]
pub enum Error {
	/// Error from the ureq agent.
	#[error(transparent)]
	AgentError(#[from] ureq::Error),
	/// Error from the Porkbun API.
	#[error(transparent)]
	ApiError(#[from] ApiError)
}
