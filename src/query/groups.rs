use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::{Authentication, Pagination};

/// Gets information about a specific group
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Group {
	/// The ID of the group to get information about
	pub id: crate::id::Group,
}

impl Queryable<Authentication, crate::model::Group> for Group {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/groups/{}", crate::API_BASE_URI, self.id.as_ref())
	}
}

/// Gets audit logs from a specific group
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GroupAuditLogs {
	/// The ID of the group to get audit logs from
	pub id: crate::id::Group,
	/// Limits how many results are returned
	#[serde(flatten)]
	pub pagination: Pagination,
}

impl Queryable<Authentication, crate::model::GroupAuditLogs>
	for GroupAuditLogs
{
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/groups/{}/auditLogs?{}",
			crate::API_BASE_URI,
			self.id.as_ref(),
			self.pagination.to_query_str()
		)
	}
}

/// Gets the bans from a group
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GroupBans {
	/// The ID of the group
	pub id: crate::id::Group,
	/// Limits how many results are returned
	#[serde(flatten)]
	pub pagination: Pagination,
}

impl Queryable<Authentication, Vec<crate::model::GroupBan>> for GroupBans {
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/groups/{}/bans?{}",
			crate::API_BASE_URI,
			self.id.as_ref(),
			self.pagination.to_query_str()
		)
	}
}

/// Bans a user from a Group.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GroupBan {
	/// The ID of the group
	pub group_id: crate::id::Group,
	/// The ID of the user to ban
	pub user_id: crate::id::User,
}

impl Queryable<Authentication, crate::model::GroupBan> for GroupBan {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/groups/{}/bans", crate::API_BASE_URI, self.group_id.as_ref(),)
	}

	fn body(
		&self, _state: &Authentication,
	) -> Option<serde_json::Result<Vec<u8>>> {
		Some(serde_json::to_vec(
			&serde_json::json!({ "userId": self.user_id.as_ref() }),
		))
	}

	fn method(&self, _state: &Authentication) -> racal::RequestMethod {
		racal::RequestMethod::Post
	}
}

/// Un-bans a user from a Group.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GroupUnban {
	/// The ID of the group
	pub group_id: crate::id::Group,
	/// The ID of the user to un-ban
	pub user_id: crate::id::User,
}

impl Queryable<Authentication, crate::model::GroupBan> for GroupUnban {
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/groups/{}/bans/{}",
			crate::API_BASE_URI,
			self.group_id.as_ref(),
			self.user_id.as_ref()
		)
	}

	fn method(&self, _state: &Authentication) -> racal::RequestMethod {
		racal::RequestMethod::Delete
	}
}

/// Returns a Limited Group Member.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GroupMember {
	/// The ID of the group
	pub group_id: crate::id::Group,
	/// The ID of the user
	pub user_id: crate::id::User,
}

impl Queryable<Authentication, Option<crate::model::GroupMember>>
	for GroupMember
{
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/groups/{}/members/{}",
			crate::API_BASE_URI,
			self.group_id.as_ref(),
			self.user_id.as_ref()
		)
	}
}
