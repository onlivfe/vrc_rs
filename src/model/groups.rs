use either::Either;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{serde::rfc3339, OffsetDateTime};

use crate::id::User;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a VRC group
pub struct Group {
	/// The unique identifier for the group.
	pub id: String,
	/// The name of the group.
	pub name: String,
	/// The short code associated with the group.
	pub short_code: String,
	/// The discriminator for the group.
	pub discriminator: String,
	/// The description of the group.
	pub description: String,
	/// The unique identifier for the group's icon.
	pub icon_id: String,
	/// The URL of the group's icon.
	pub icon_url: String,
	/// The unique identifier for the group's banner.
	pub banner_id: String,
	/// The URL of the group's banner.
	pub banner_url: String,
	/// The privacy setting of the group.
	pub privacy: String,
	/// The unique identifier of the owner of the group.
	pub owner_id: String,
	/// The rules associated with the group.
	pub rules: String,
	/// The list of links associated with the group.
	pub links: Vec<String>,
	/// The list of languages associated with the group.
	pub languages: Vec<String>,
	/// The count of members in the group.
	pub member_count: i64,
	/// The times tamp when the member count was last synchronized.
	pub member_count_synced_at: String,
	/// Indicates whether the group is verified.
	pub is_verified: bool,
	/// The join state of the group.
	pub join_state: String,
	// pub tags: Vec<Value>,
	// pub galleries: Vec<Value>,
	/// The time stamp when the group was created.
	pub created_at: String,
	/// The count of online members in the group.
	pub online_member_count: i64,
	/// The membership status of the user.
	pub membership_status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Represents a collection of group audit logs.
pub struct GroupAuditLogs {
	/// The list of group audit logs.
	pub results: Vec<GroupAuditLog>,
	/// The total count of audit logs.
	pub total_count: u32,
	/// Indicates whether there are more audit logs.
	pub has_next: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Represents a single group audit log entry.
pub struct GroupAuditLog {
	/// The unique identifier for the audit log entry.
	pub id: String,
	/// The time stamp when the audit log entry was created.
	#[serde(rename = "created_at", with = "rfc3339")]
	pub created_at: OffsetDateTime,
	/// The unique identifier of the group associated with the audit log.
	pub group_id: String,
	/// The unique identifier of the actor who performed the action.
	pub actor_id: User,
	/// The display name of the actor.
	pub actor_displayname: Option<String>,
	/// The unique identifier of the target of the action.
	pub target_id: Option<User>,
	/// The type of event captured in the audit log.
	pub event_type: String,
	/// The description of the event captured in the audit log.
	pub description: String,
	/// Additional data associated with the audit log entry.
	pub data: GroupAuditLogData,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Represents additional data associated with a group audit log entry.
pub struct GroupAuditLogData {
	/// The description change associated with the audit log entry.
	#[serde(default, with = "either::serde_untagged_optional")]
	pub description: Option<Either<GroupAuditLogDataChange<String>, String>>,
	/// The join state change associated with the audit log entry.
	#[serde(default, with = "either::serde_untagged_optional")]
	pub join_state: Option<Either<GroupAuditLogDataChange<String>, String>>,
	/// The order change associated with the audit log entry.
	#[serde(default, with = "either::serde_untagged_optional")]
	pub order: Option<Either<GroupAuditLogDataChange<u32>, u32>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Represents a change in field associated with a group audit log entry.
pub struct GroupAuditLogDataChange<T> {
	/// The old field before the change.
	pub old: T,
	/// The new field after the change.
	pub new: T,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a group (un)ban.
pub struct GroupBan {
	/// Unique identifier of the (un)ban.
	pub id: String,
	/// Identifier of the group.
	pub group_id: crate::id::Group,
	/// Identifier of the user who was (un)banned.
	pub user_id: crate::id::User,
	/// Flag indicating if the user was representing the group at the time of
	/// (un)ban.
	pub is_representing: bool,
	/// List of role identifiers the user had in the group.
	pub role_ids: Vec<Value>,
	/// List of managed role identifiers the user had in the group.
	pub m_role_ids: Vec<Value>,
	/// Timestamp of when the user joined.
	pub joined_at: Option<String>,
	/// Status of the user's membership in the group at the time of (un)ban.
	pub membership_status: String,
	/// Visibility status of the user in the group.
	pub visibility: String,
	/// Flag indicating if the user was subscribed to group announcements.
	pub is_subscribed_to_announcements: bool,
	/// Timestamp of the last post read by the user in the group.
	pub last_post_read_at: Option<Value>,
	/// Timestamp when the user joined the group.
	pub created_at: String,
	/// Timestamp when the user was (un)banned from the group.
	pub banned_at: Option<String>,
	/// Notes added by the group manager regarding the ban.
	pub manager_notes: String,
	/// Flag indicating if the user joined the group from a purchase.
	pub has_joined_from_purchase: bool,
}
