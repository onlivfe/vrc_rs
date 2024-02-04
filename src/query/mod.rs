//! Models of the queries for VRC's API.

#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};
use strum::AsRefStr;

mod auth;
pub use auth::*;
mod friends;
pub use friends::*;
mod groups;
pub use groups::*;
mod instances;
pub use instances::*;
mod users;
pub use users::*;
mod worlds;
pub use worlds::*;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to VRC's API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct Authenticating {
	/// The username to login with
	pub username: String,
	/// The password to login with
	pub password: String,
}

impl std::fmt::Debug for Authenticating {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Authenticating")
			.field("username", &self.username)
			.field("password", &"*****")
			.finish()
	}
}

impl racal::FromApiState<Self> for Authenticating {
	fn from_state(state: &Self) -> &Self { state }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// With authentication
pub struct Authentication {
	/// Normal auth cookie
	pub token: String,
	/// 2FA remember cookie
	pub second_factor_token: Option<String>,
}

impl std::fmt::Debug for Authentication {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Authentication")
			.field("token", &"*****")
			.field(
				"second_factor_token",
				match &self.second_factor_token {
					Some(_) => &"Some(*****)",
					None => &"None",
				},
			)
			.finish()
	}
}

impl racal::FromApiState<Self> for Authentication {
	fn from_state(state: &Self) -> &Self { state }
}

/// Ordering for a query
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, AsRefStr,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum Order {
	/// Ascending ordering, so first is first
	Ascending,
	/// Descending ordering, so last is first
	Descending,
}

impl Default for Order {
	fn default() -> Self { Self::Ascending }
}

/// Gets the friends list
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Pagination {
	#[serde(rename = "n")]
	/// The max amount of items to return
	pub limit: u8,
	/// Which page to get
	pub offset: u32,
}

impl Default for Pagination {
	fn default() -> Self { Self { limit: 10, offset: 0 } }
}

impl Pagination {
	fn to_query_str(&self) -> String {
		format!("n={}&offset={}", self.limit, self.offset)
	}
}
