//! Errors and error handling QOL.

use thiserror::Error;

use crate::ErrorMessage;

/// Actual error to return when [ApiResponse][crate::response::ApiResponse] returns an error.
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub struct Error {
	pub status: ureq::http::StatusCode,
	pub error: Option<ErrorMessage>
}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(error) = &self.error {
			write!(f, "{}: {}", self.status, error)
		} else {
			write!(f, "{}", self.status)
		}
	}
}

#[derive(Error, Debug)]
pub enum ApiError {
	#[error(transparent)]
	AgentError(#[from] ureq::Error),
	#[error(transparent)]
	ApiError(#[from] Error)
}
