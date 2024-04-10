use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::Authentication;

/// Gets information about a specific user account
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct User {
	/// The ID of the account to get information about
	pub id: crate::id::User,
}

impl Queryable<Authentication, crate::model::AnyUser> for User {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/users/{}", crate::API_BASE_URI, self.id.as_ref())
	}
}

/// Search and list any users by text query
#[derive(
	Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize,
)]
pub struct SearchUser {
	/// Searches by display name. Will return empty array if search query is empty
	/// or missing.
	pub search: String,
	/// The number of objects to return.
	pub n: Option<u8>,
	/// A zero-based offset from the default object sorting from where search
	/// results start.
	pub offset: Option<usize>,
}

impl Queryable<Authentication, Vec<crate::model::AccountData>> for SearchUser {
	fn url(&self, _: &Authentication) -> String {
		let mut query =
			format!("{}/users?search={}", crate::API_BASE_URI, self.search,);

		if let Some(n) = &self.n {
			query.push_str("&n=");
			query.push_str(&n.to_string());
		}

		if let Some(offset) = &self.offset {
			query.push_str("&offset=");
			query.push_str(&offset.to_string());
		}

		query
	}
}
