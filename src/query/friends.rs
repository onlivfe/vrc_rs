use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::Authentication;

/// Gets the friends list
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ListFriends {
	/// The max amount of friends to return
	pub limit: u8,
	/// Which page to get
	pub offset: u32,
}

impl Default for ListFriends {
	fn default() -> Self { Self { limit: 10, offset: 0 } }
}

impl Queryable<Authentication, Vec<crate::model::Friend>> for ListFriends {
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/auth/user/friends?n={}&offset={}",
			crate::API_BASE_URI,
			self.limit,
			self.offset
		)
	}
}
