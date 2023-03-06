//! Models of the API responses to queries.

// Can't do much about API types
#![allow(clippy::struct_excessive_bools)]

mod instances;
pub use instances::*;
mod notifications;
pub use notifications::*;
mod users;
use serde::{Deserialize, Serialize};
pub use users::*;
mod tags;
pub use tags::*;

/// A generic success status response
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SuccessResponse {
	message: String,
	status_code: u16,
}

/// A very generic response to some operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StatusResponse {
	/// The operation was successful
	Success(SuccessResponse),
}
