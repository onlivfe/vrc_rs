use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};
use url::Url;

use crate::id::{OfflineOr, OfflineOrPrivateOr};

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
	fn default() -> Self { Self::None }
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
	fn default() -> Self { Self::Offline }
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
	/// Also known as gray
	Offline,
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
pub enum FriendRequestStatus {
	/// The user has requested to become friends with the authenticated user
	Incoming,
	/// Also known as orange
	Outgoing,
	/// No pending friend requests (or serialization failed...)
	#[serde(other)]
	Other,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Presence details for the authenticated account
pub struct Presence {
	/// The URL of the avatar's thumbnail
	pub avatar_thumbnail: Option<Url>,
	#[serde(default)]
	/// The display name of the user
	///
	/// Can be empty string if missing
	pub display_name: String,
	/// The groups that the user is in
	pub groups: Vec<serde_json::Value>,
	/// The ID of the user
	pub id: crate::id::User,
	/// The instance that the user is in
	pub instance: crate::id::OfflineOr<crate::id::Instance>,
	#[serde(default)]
	/// The instance type that the user is in.
	///
	/// Can be empty string if missing.
	pub instance_type: String,
	#[serde(default)]
	/// The platform that the user is on.
	///
	/// Can be empty string if missing.
	pub platform: String,
	/// The user's own picture to replace the avatar pic with
	#[serde(default)]
	#[serde_as(as = "serde_with::NoneAsEmptyString")]
	pub profile_pic_override: Option<Url>,
	/// The status of the user
	pub status: UserStatus,
	/// If the user is traveling to a new instance
	pub traveling_to_instance: crate::id::OfflineOrPrivateOr<crate::id::Instance>,
	/// If the user is traveling to a new world
	pub traveling_to_world: crate::id::OfflineOrPrivateOr<crate::id::World>,
	/// The world that the user is in
	pub world: crate::id::OfflineOr<String>,
	/// If the user is rejoining the instance
	#[serde(default)]
	pub is_rejoining: serde_json::Value,
	/// The current avatar tags
	#[serde(default)]
	pub current_avatar_tags: serde_json::Value,
	/// URL to the user's icon, can be an empty string
	#[serde(default)]
	#[serde_as(as = "serde_with::NoneAsEmptyString")]
	pub user_icon: Option<Url>,
	/// What does this do?
	#[serde(default)]
	pub debugflag: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Presence details for the authenticated account
pub struct PastDisplayName {
	/// The display name itself
	pub display_name: String,
	/// When the entry was updated
	#[serde(rename = "updated_at", with = "rfc3339")]
	pub updated_at: OffsetDateTime,
	/// If the display name change has been reverted
	#[serde(default)]
	pub reverted: bool,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Some details about a VRC user
pub struct AccountData {
	#[serde(default)]
	/// Text that the user has written about themselves
	///
	/// Defaults to an empty string if missing
	pub bio: String,
	/// Links that the user has added about themselves
	pub bio_links: Vec<String>,
	/// The avatar's image, for profile picture see the [profile pic
	/// override](Self::profile_pic_override)
	pub current_avatar_image_url: Url,
	/// The avatar's smaller image, for profile picture see the [profile pic
	/// override](Self::profile_pic_override)
	pub current_avatar_thumbnail_image_url: Url,
	/// If the user has some sort of a special status
	pub developer_type: DeveloperType,
	/// A users visual display name. This is what shows up in-game, and can
	/// different from their `username`. Changing display name is restricted to
	/// a cool down period.
	pub display_name: String,
	/// The user's ID
	pub id: crate::id::User,
	/// If the user is a friend of the currently authenticated user
	pub is_friend: bool,
	// TODO: Platform enum
	/// This can be `standalonewindows` or `android`, but can also pretty much
	/// be any random Unity version or even `unknownplatform`.
	#[serde(rename = "last_platform")]
	pub last_platform: String,
	/// Possible profile picture URL
	#[serde(default)]
	#[serde_as(as = "serde_with::NoneAsEmptyString")]
	pub profile_pic_override: Option<Url>,
	/// The status of the user
	pub status: UserStatus,
	/// User set status message
	pub status_description: String,
	/// Tags of the user
	pub tags: Vec<String>,
	/// URL to the user's icon, can be an empty string
	#[serde(default)]
	#[serde_as(as = "serde_with::NoneAsEmptyString")]
	pub user_icon: Option<Url>,
}

/// Details that get added if the user is the authenticated one
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAccountData {
	/// Which version of the TOS the authenticated user has accepted
	#[serde(rename = "acceptedTOSVersion")]
	pub accepted_tos_version: u8,
	/// Which version of the Privacy Policy the authenticated user has accepted
	pub accepted_privacy_version: u8,
	/// The URL to the current avatar's asset
	pub current_avatar_asset_url: Url,
	/// If the email address of the account has been verified
	pub email_verified: bool,
	/// Names of friend groups?
	pub friend_group_names: Vec<serde_json::Value>,
	/// Details of when the account has been deleted
	pub account_deletion_date: serde_json::Value,
	/// A log of the account deletion
	pub account_deletion_log: serde_json::Value,
	/// The friends of the user
	pub friends: Vec<crate::id::User>,
	/// If the user has provided their birthday
	pub has_birthday: bool,
	/// If the user has provided their email
	pub has_email: bool,
	/// If the user has logged in from a game client
	pub has_logged_in_from_client: bool,
	/// If the user has a pending email
	pub has_pending_email: bool,
	/// The location of the home, or an empty string
	#[serde(default)]
	#[serde_as(as = "serde_with::NoneAsEmptyString")]
	pub home_location: Option<crate::id::World>,
	#[serde(default)]
	/// Can presumably be an empty string
	pub obfuscated_email: String,
	/// Can be an empty string
	#[serde(default)]
	pub obfuscated_pending_email: String,
	/// Can be empty
	#[serde(default)]
	pub past_display_names: Vec<PastDisplayName>,
	/// If hasn't set status yet
	pub status_first_time: bool,
	/// History of statuses (VRC pre-populates some for new accounts)
	pub status_history: Vec<String>,
	/// Details of the linked steam account
	pub steam_details: serde_json::Value,
	/// Can be empty
	#[serde(default)]
	pub steam_id: String,
	/// Can be an empty string
	#[serde(default)]
	pub google_id: String,
	/// Can be an empty string
	#[serde(default)]
	pub oculus_id: String,
	/// Can be an empty string
	#[serde(default)]
	pub pico_id: String,
	/// Can be an empty string
	#[serde(default)]
	pub vive_id: String,
	/// If 2FA is enabled
	pub two_factor_auth_enabled: bool,
	#[serde(default)]
	#[serde(with = "rfc3339::option")]
	/// When 2FA was enabled
	pub two_factor_auth_enabled_date: Option<OffsetDateTime>,
	/// If unsubscribed from marketing emails probably
	pub unsubscribe: bool,
	#[serde(default)]
	/// Should not be relied upon, [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429)
	pub username: String,
}

/// Data that's returned about friends that's returned from the friends and
/// users endpoints
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendData {
	/// Nobody seems to know what this is for
	pub friend_key: String,
	/// Either a date-time or empty string.
	#[serde(rename = "last_login", with = "rfc3339")]
	pub last_login: OffsetDateTime,
	// If the user is traveling to somewhere
	//pub traveling_to_location: OfflineOrPrivateOr<String>,
	/// The location of the friend
	// Not included for users since itd always just be "offline"
	pub location: OfflineOrPrivateOr<crate::id::Instance>,
}

/// Data that's returned from the users endpoint about friends or the
/// authenticated user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusData {
	/// The user's state
	pub state: UserState,
	/// when the last activity for the account was at
	#[serde(rename = "last_activity", with = "rfc3339")]
	pub last_activity: OffsetDateTime,
}

/// Data that's returned from the users endpoint about friends or the
/// authenticated user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatusData {
	/// Base status data that's also for current account endpoint
	#[serde(flatten)]
	pub base: StatusData,
	/// If the user is traveling to somewhere
	pub traveling_to_instance: OfflineOrPrivateOr<crate::id::Instance>,
	/// If the user is traveling to somewhere
	pub traveling_to_world: OfflineOrPrivateOr<crate::id::World>,
	/// The location of the friend
	pub location: OfflineOrPrivateOr<crate::id::Instance>,
	/// The current instance's ID, or offline
	// Not included on User since it'd always be "offline"
	pub instance_id: OfflineOr<crate::id::Instance>,
	/// The current world's ID
	pub world_id: OfflineOr<crate::id::World>,
}

/// Extended details about the current user
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAccount {
	/// Base info that's shared across different user responses
	#[serde(flatten)]
	pub base: AccountData,
	#[serde(flatten)]
	/// Details about the current user as an authenticated account
	pub current: CurrentAccountData,
	/// Base status data that's also for user endpoint
	#[serde(flatten)]
	pub status: StatusData,
	/// The presence of self
	pub presence: Presence,
	/// Friends that are offline
	pub offline_friends: Vec<crate::id::User>,
	/// Friends that are online
	pub online_friends: Vec<crate::id::User>,
	/// Friends that are active
	pub active_friends: Vec<crate::id::User>,
	/// Either a date-time or empty string.
	#[serde(rename = "last_login", with = "rfc3339")]
	pub last_login: OffsetDateTime,
	/// When the entry was updated
	#[serde(rename = "updated_at", with = "rfc3339")]
	pub updated_at: OffsetDateTime,
	/// The current avatar id or empty string.
	#[serde(default)]
	pub current_avatar: String,
	/// The current avatar tags or empty string.
	#[serde(default)]
	pub current_avatar_tags: serde_json::Value,
	/// The fallback avatar id or empty string.
	#[serde(default)]
	pub fallback_avatar: String,
}

/// Information that's returned about friends
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
	/// Base info that's shared across different user responses
	#[serde(flatten)]
	pub base: AccountData,
	/// Data about the friend
	#[serde(flatten)]
	pub friend: FriendData,
	// The fallback avatar's ID
	//pub fallback_avatar: crate::id::Avatar,
	/// The user's image
	pub image_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a VRC user
pub struct User {
	/// Base info that's shared across different user responses
	#[serde(flatten)]
	pub base: AccountData,
	/// If the user has avatar cloning on
	pub allow_avatar_copying: bool,
	/// When the user joined VRC
	#[serde(rename = "date_joined", with = "crate::date_format")]
	pub date_joined: time::Date,
	/// The friend request status with this user
	pub friend_request_status: FriendRequestStatus,
	/// Notes about the user
	pub note: String,
}

/// Data that's returned from the user endpoint about the authenticated user
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
	/// Base user info
	#[serde(flatten)]
	pub base: User,
	#[serde(flatten)]
	/// Extended details show due to authentication
	pub account: CurrentAccountData,
	#[serde(flatten)]
	/// Status data that's also shown about friends
	pub status: UserStatusData,
	#[serde(flatten)]
	/// Data that's shared with friends response
	pub friend: FriendData,
	/// The fallback avatar
	pub fallback_avatar: crate::id::Avatar,
}

/// Data that's returned from the user endpoint about the authenticated user's
/// friends
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendUser {
	/// Base user data
	#[serde(flatten)]
	pub base: User,
	#[serde(flatten)]
	/// Status data that's also shown about friends
	pub status: UserStatusData,
	#[serde(flatten)]
	/// Data that's shared with friends response
	pub friend: FriendData,
}

/// Data that's returned from the user endpoint
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnyUser {
	/// Full data that's returned for the authenticated user
	Authenticated(Box<CurrentUser>),
	/// Data that's returned about friends
	Friend(Box<FriendUser>),
	/// Data that's returned about all users
	User(Box<User>),
}

impl AnyUser {
	#[must_use]
	/// Borrows the base user struct from whatever kind of user it is
	pub const fn as_user(&self) -> &User {
		match &self {
			Self::Authenticated(a) => &a.base,
			Self::Friend(f) => &f.base,
			Self::User(u) => u,
		}
	}

	#[must_use]
	/// Unwraps the base user struct from whatever kind of user it is
	pub fn into_user(self) -> User {
		match self {
			Self::Authenticated(a) => a.base,
			Self::Friend(f) => f.base,
			Self::User(u) => *u,
		}
	}

	#[must_use]
	/// Borrows the base user struct from whatever kind of user it is
	pub const fn status(&self) -> Option<&UserStatusData> {
		match &self {
			Self::Authenticated(a) => Some(&a.status),
			Self::Friend(f) => Some(&f.status),
			Self::User(_) => None,
		}
	}

	#[must_use]
	/// Borrows the base user struct from whatever kind of user it is
	pub const fn friend(&self) -> Option<&FriendData> {
		match &self {
			Self::Authenticated(a) => Some(&a.friend),
			Self::Friend(f) => Some(&f.friend),
			Self::User(_) => None,
		}
	}
}
