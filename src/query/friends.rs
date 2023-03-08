use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::{Authentication, Pagination};

/// Gets the friends list
#[derive(
	Default, Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
pub struct ListFriends {
	/// The pagination for the query
	#[serde(flatten)]
	pub pagination: Pagination,
	#[serde(rename = "offline")]
	/// If to filter based on the offline/online status,
	/// true for offline users and false for online users
	pub status_filter: Option<bool>,
}

impl Queryable<Authentication, Vec<crate::model::Friend>> for ListFriends {
	fn url(&self, _: &Authentication) -> String {
		let mut query = format!(
			"{}/auth/user/friends?{}",
			crate::API_BASE_URI,
			self.pagination.to_query_str()
		);

		if let Some(status_filter) = self.status_filter {
			query.push_str("&offline=");
			query.push_str(&status_filter.to_string());
		}

		query
	}
}
