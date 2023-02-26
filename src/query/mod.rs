//! Models of the queries for VRC's API.

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to VRC's API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct NoAuthentication {}

impl From<&Self> for NoAuthentication {
	fn from(_: &Self) -> Self {
		Self {}
	}
}

impl From<&Authentication> for NoAuthentication {
	fn from(_: &Authentication) -> Self {
		Self {}
	}
}

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// With authentication
pub struct Authentication {
	/// The secret authentication token
	pub token: String,
	/// The user that the authentication token is for
	pub user_id: crate::id::User,
}

impl From<&Self> for Authentication {
	fn from(auth: &Self) -> Self {
		Self { token: auth.token.clone(), user_id: auth.user_id.clone() }
	}
}
