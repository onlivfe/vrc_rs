use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InstancePrivacy {
	/// Public instance
	Public,
	/// Also known as `Friends+`, in the API it's called "hidden"
	#[serde(rename = "hidden")]
	FriendsOfFriends,
	/// Friends only
	Friends,
	/// Invite or Invite+
	Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum InstanceRegion {
	#[serde(rename = "us")]
	Usa,
	#[serde(rename = "usw")]
	UsaWest,
	#[serde(rename = "use")]
	UsaEast,
	#[serde(rename = "eu")]
	Europe,
	#[serde(rename = "jp")]
	Japan,
	#[serde(other)]
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about the instance creator that only exist if current user is
/// treated as the creator
pub enum InstanceCreatorId {
	/// The instance type is Friends+
	Hidden(crate::model::id::User),
	/// The instance type is Friends
	Friends(crate::model::id::User),
	/// The instance type is Invite or Invite+
	Private(crate::model::id::User),
	#[serde(other)]
	None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct InstancePlatforms {
	pub android: i32,
	pub standalonewindows: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
	pub active: bool,
	/// If an invite can be requested to the instance
	pub can_request_invite: bool,
	pub capacity: i32,
	/// If the instance is considered to be filled to the user capacity or not
	pub full: bool,
	#[serde(rename = "id")]
	/// Set to offline if on an user profile that the authenticated user is not
	/// friends with
	pub instance_id: crate::model::id::OfflineOrId<crate::model::id::Instance>,
	#[serde(rename = "n_users")]
	pub user_count: i32,
	pub name: String,
	pub owner_id: Option<crate::model::id::User>,
	/// If the instance is supposedly permanent?
	pub permanent: bool,
	/// Apparently the photon region can theoretically be different than the
	/// actual region?
	#[serde(rename = "photonRegion")]
	pub photon_region: InstanceRegion,
	pub platforms: InstancePlatforms,
	#[serde(rename = "region")]
	pub region: InstanceRegion,
	#[serde(rename = "secureName")]
	pub secure_name: String,
	pub short_name: Option<String>,
	pub tags: Vec<crate::model::Tag>,
	#[serde(rename = "type")]
	pub privacy: InstancePrivacy,
	/// Will be offline on User profiles if not friends with the creator
	pub world_id: crate::model::id::OfflineOrId<crate::model::id::World>,
	/// An ID of the world creator that only exists if current user is treated
	/// as the creator
	#[serde(flatten)]
	pub creator_details: InstanceCreatorId,
}
