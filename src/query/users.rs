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
