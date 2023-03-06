use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::Authentication;

/// Gets the friends list
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ListFriends;

impl Queryable<Authentication, Vec<crate::model::Friend>> for ListFriends {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/auth/user/friends", crate::API_BASE_URI)
	}
}
