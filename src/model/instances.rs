use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Who can join the instance
pub enum InstancePrivacy {
	/// Public instance
	Public,
	/// Also known as `Friends+`, in the API it's called "hidden"
	#[serde(rename = "hidden")]
	FriendsOfFriends,
	/// Friends only
	Friends,
	/// A Group instance
	Group,
	/// Invite or Invite Plus
	Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// The region of the instance
pub enum InstanceRegion {
	#[serde(rename = "us")]
	/// Anywhere within the USA
	Usa,
	#[serde(rename = "usw")]
	/// Hosted in San José
	UsaWest,
	#[serde(rename = "use")]
	/// Hosted in Washington D.C.
	UsaEast,
	#[serde(rename = "eu")]
	/// Hosted in Amsterdam
	Europe,
	#[serde(rename = "jp")]
	/// Hosted in Tokyo
	Japan,
	#[serde(other)]
	/// Possible other values that may get added in the future
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about the instance creator that only exist if current user is
/// treated as the creator
pub enum InstanceCreatorId {
	/// The instance type is Friends Plus
	Hidden(crate::id::User),
	/// The instance type is Friends
	Friends(crate::id::User),
	/// The instance type is Invite or Invite Plus
	Private(crate::id::User),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// How many users are on each platform
pub struct InstancePlatformUserCounter {
	/// How many quest users there are
	pub android: u16,
	/// How many desktop users there are
	#[serde(rename = "standalonewindows")]
	pub windows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about an instance
pub struct Instance {
	/// If the instance is still ongoing or historical
	pub active: bool,
	/// If an invite can be requested to the instance
	pub can_request_invite: bool,
	/// How many users can fit into the instance
	pub capacity: u32,
	/// If the instance is considered to be filled to the user capacity or not
	pub full: bool,
	/// The ID of the world & instance
	pub id: crate::id::WorldInstance,
	/// The ID of the instance
	pub instance_id: crate::id::Instance,
	#[serde(rename = "n_users")]
	/// How many users are currently in the instance
	pub user_count: u32,
	/// The name of the instance
	pub name: String,
	/// The ID of the instance's owner
	pub owner_id: Option<crate::id::User>,
	/// If the instance is supposedly permanent
	pub permanent: bool,
	/// Apparently the photon region can theoretically be different than the
	/// actual region?
	pub photon_region: InstanceRegion,
	/// How many users there are on each platform
	pub platforms: InstancePlatformUserCounter,
	/// The region that the instance is running in
	pub region: InstanceRegion,
	#[serde(rename = "secureName")]
	/// ... name of the instance, but secure..?
	pub secure_name: String,
	/// ...shorter name for the instance?
	pub short_name: Option<String>,
	/// Tags for the instance
	pub tags: Vec<String>,
	#[serde(rename = "type")]
	/// Who can join the instance, also called instance type
	pub privacy: InstancePrivacy,
	/// The ID of the world that the instance is running
	pub world_id: crate::id::World,
	/// An ID of the world creator that only exists if current user is treated
	/// as the creator
	#[serde(flatten, default)]
	pub creator_details: Option<InstanceCreatorId>,
}
