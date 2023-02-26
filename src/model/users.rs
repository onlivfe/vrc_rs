use serde::{Deserialize, Serialize};

#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
)]
#[serde(rename_all = "camelCase")]
/// If the user has some special status
pub enum DeveloperType {
	/// No special status
	None,
	/// The user is trusted by VRC developers?
	Trusted,
	/// The user is a developer
	Internal,
	/// The user is a moderator
	Moderator,
}

impl Default for DeveloperType {
	fn default() -> Self {
		Self::None
	}
}

#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
)]
#[serde(rename_all = "camelCase")]
/// If the user is offline or not
pub enum UserState {
	/// The user is offline
	Offline,
	/// The user is on the website for example
	Active,
	/// User is online on VRC
	Online,
}

impl Default for UserState {
	fn default() -> Self {
		Self::Offline
	}
}

#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
)]
#[serde(rename_all = "camelCase")]
/// The status of an user
pub enum UserStatus {
	/// Also known as green
	Active,
	#[serde(rename = "join me")]
	/// Also known as blue
	JoinMe,
	#[serde(rename = "ask me")]
	/// Also known as orange
	AskMe,
	/// Also known as red
	Busy,
	/// Also known as gray
	Offline,
}

impl Default for UserStatus {
	fn default() -> Self {
		Self::Offline
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Details about a VRC user
pub struct User {
	/// If the user has avatar cloning on
	pub allow_avatar_copying: bool,
	/// Text that the user has written about themselves
	pub bio: String,
	/// Links that the user has added about themselves
	pub bio_links: Vec<String>,
	/// The avatar's image, for profile picture see the [profile pic override](Self::profile_pic_override)
	pub current_avatar_image_url: String,
	/// The avatar's smaller image, for profile picture see the [profile pic override](Self::profile_pic_override)
	pub current_avatar_thumbnail_image_url: String,
	#[serde(rename = "date_joined")]
	/// When the user joined VRC
	pub date_joined: String,
	/// If the user has some sort of a special status
	pub developer_type: DeveloperType,
	/// A users visual display name. This is what shows up in-game, and can different from their `username`. Changing display name is restricted to a cool down period.
	pub display_name: String,
	/// TODO: Figure out
	pub friend_key: String,
	/// TODO: Figure out
	pub friend_request_status: String,
	/// The user's ID
	pub id: crate::id::User,
	/// The ID that the user is in
	pub instance_id: crate::id::OfflineOr<crate::id::Instance>,
	/// If the user is a friend of the currently authenticated user
	pub is_friend: bool,
	/// Either a date-time or empty string.
	#[serde(rename = "last_activity")]
	pub last_activity: String,
	/// Either a date-time or empty string.
	#[serde(rename = "last_login")]
	pub last_login: String,
	// TODO: Platform enum
	/// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity version or even `unknownplatform`.
	#[serde(rename = "last_platform")]
	pub last_platform: String,
	/// The location that the user is in
	pub location: crate::id::OfflineOr<crate::id::World>,
	/// The currently authenticated user's note about this user
	pub note: String,
	/// Possible profile picture URL
	pub profile_pic_override: String,
	/// If the user is online or not
	pub state: UserState,
	/// The status of the user
	pub status: UserStatus,
	/// Human readable version of the status
	pub status_description: String,
	// TODO: Enum
	/// Tags of the user
	pub tags: Vec<String>,
	/// If the user is traveling to an instance but not yet in it
	pub traveling_to_instance: Option<crate::id::Instance>,
	/// If the user is traveling to somewhere but not yet there
	pub traveling_to_location: Option<String>,
	/// If the user is traveling to a world but not yet in it
	pub traveling_to_world: Option<crate::id::World>,
	/// URL to the user's icon
	pub user_icon: String,
	/// Only returned if for current user. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429).
	#[serde(rename = "username", skip_serializing_if = "Option::is_none")]
	pub username: Option<String>,
	/// The ID of the world that the user is in
	pub world_id: crate::id::OfflineOr<crate::id::World>,
}
