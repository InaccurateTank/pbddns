//! Errors and error handling QOL.

use thiserror::Error;

use crate::response::ErrorMessage;

/// Actual error to return when [ApiResponse][crate::response::ApiResponse] returns an error.
#[derive(Error, Debug)]
#[allow(missing_docs)]
#[error("{status}: {error:?}")]
pub struct Error {
	pub status: http::StatusCode,
	pub error: Option<ErrorMessage>
}
