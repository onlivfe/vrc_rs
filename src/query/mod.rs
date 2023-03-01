//! Models of the queries for VRC's API.

mod auth;
pub use auth::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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

impl From<&Self> for Authenticating {
	fn from(auth: &Self) -> Self {
		Self { username: auth.username.clone(), password: auth.password.clone() }
	}
}

#[derive(Clone, Serialize, Deserialize)]
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
			.field("token", &self.token)
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

impl From<&Self> for Authentication {
	fn from(auth: &Self) -> Self {
		Self {
			token: auth.token.clone(),
			second_factor_token: auth.second_factor_token.clone(),
		}
	}
}
