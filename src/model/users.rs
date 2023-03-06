use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Some details about a VRC user
pub struct UserBase {
	#[serde(default)]
	/// Text that the user has written about themselves
	///
	/// Defaults to an empty string if missing
	pub bio: String,
	/// Links that the user has added about themselves
	pub bio_links: Vec<String>,
	/// The avatar's image, for profile picture see the [profile pic
	/// override](Self::profile_pic_override)
	pub current_avatar_image_url: String,
	/// The avatar's smaller image, for profile picture see the [profile pic
	/// override](Self::profile_pic_override)
	pub current_avatar_thumbnail_image_url: String,
	/// If the user has some sort of a special status
	pub developer_type: DeveloperType,
	/// A users visual display name. This is what shows up in-game, and can
	/// different from their `username`. Changing display name is restricted to
	/// a cool down period.
	pub display_name: String,
	#[serde(default)]
	/// TODO: Figure out
	///
	/// Defaulted to an empty string if missing
	pub friend_key: String,
	/// The user's ID
	pub id: crate::id::User,
	// TODO: Replace this & serde flattens with proper enum
	/// If the user is a friend of the currently authenticated user
	pub is_friend: bool,
	// TODO: Platform enum
	/// This can be `standalonewindows` or `android`, but can also pretty much
	/// be any random Unity version or even `unknownplatform`.
	#[serde(rename = "last_platform")]
	pub last_platform: String,
	/// Possible profile picture URL
	pub profile_pic_override: String,
	/// The status of the user
	pub status: UserStatus,
	/// User set status message
	pub status_description: String,
	/// Tags of the user
	pub tags: Vec<String>,
	/// URL to the user's icon, can be an empty string
	#[serde(default)]
	pub user_icon: String,
}

/// Details that get added if the user is the authenticated one
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithAuthenticationDetails {
	/// Which version of the TOS the authenticated user has accepted
	#[serde(rename = "acceptedTOSVersion")]
	pub accepted_tos_version: u8,
	// pub accountDeletionDate: null,
	// pub accountDeletionLog: null,
	/// If the email address of the account has been verified
	pub email_verified: bool,
	//pub friendGroupNames: Vec<>,
	// pub friendRequestStatus: "null"
	//pub accountDeletionDate: null,
	//pub accountDeletionLog: null,
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
	// TODO: serde_with empty str to Optioon<rate::id::instance>
	/// The location of the home, or an empty string
	pub home_location: String,
	/// The current instance's ID, or offline
	pub instance_id: OfflineOr<crate::id::Instance>,
	#[serde(default)]
	/// Can presumably be an empty string
	pub obfuscated_email: String,
	#[serde(default)]
	/// Can be an empty string
	pub obfuscated_pending_email: String,
	/// Can be an empty string
	#[serde(default)]
	pub oculus_id: String,
	#[serde(default)]
	/// Can be empty
	pub past_display_names: Vec<String>,
	/// If hasn't set status yet
	pub status_first_time: bool,
	/// History of statuses (VRC pre-populates some for new accounts)
	pub status_history: Vec<String>,
	//pub steam_etails: {}
	#[serde(default)]
	/// Can be empty
	pub steam_id: String,
	/// If 2FA is enabled
	pub two_factor_auth_enabled: bool,
	// TODO: combine with option, #[serde(with = "rfc3339")]
	//pub twoFactorAuthEnabledDate: Option<OffsetDateTime>,
	/// If unsubscribed from marketing emails probably
	pub unsubscribe: bool,
	#[serde(default)]
	/// Should not be relied upon, [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429)
	pub username: String,
	/// The current world's ID
	pub world_id: OfflineOr<crate::id::World>,
}

/// Details that get added if the user is friends with the authenticated one
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithFriendDetails {
	/// Either a date-time or empty string.
	#[serde(rename = "last_login", with = "rfc3339")]
	pub last_login: OffsetDateTime,
	/// Either a date-time or empty string.
	#[serde(rename = "last_activity", with = "rfc3339")]
	pub last_activity: OffsetDateTime,
	/// If the user is traveling to somewhere
	pub traveling_to_instance: OfflineOrPrivateOr<crate::id::Instance>,
	/// If the user is traveling to somewhere
	pub traveling_to_location: OfflineOrPrivateOr<String>,
	/// If the user is traveling to somewhere
	pub traveling_to_world: OfflineOrPrivateOr<crate::id::World>,
	/// The location of the friend
	pub location: OfflineOrPrivateOr<crate::id::Instance>,
}

/// Extended details about the current user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
	/// Base info that's shared across different user responses
	#[serde(flatten)]
	pub user: UserBase,
	#[serde(flatten)]
	/// Details about the current user as a friend
	pub friend: UserWithFriendDetails,
	#[serde(flatten)]
	/// Details about the current user as an authenticated account
	pub current: UserWithAuthenticationDetails,
	/// The friend's state
	pub state: UserState,
}

/// Information that's returned about friends
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
	/// Base info that's shared across different user responses
	#[serde(flatten)]
	pub base: UserBase,
	/// Either a date-time or empty string.
	#[serde(rename = "last_login", with = "rfc3339")]
	pub last_login: OffsetDateTime,
	/// TODO: Figure out
	pub fallback_avatar: crate::id::Avatar,
	/// The location of the friend
	pub location: OfflineOrPrivateOr<crate::id::Instance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a VRC user
pub struct User {
	/// Base info that's shared across different user responses
	#[serde(flatten)]
	pub base: UserBase,
	#[serde(flatten)]
	/// Possible extended details if the current user
	pub current: Option<UserWithAuthenticationDetails>,
	#[serde(flatten)]
	/// Possible extended details if it's a friend of the current user
	pub friend: Option<UserWithFriendDetails>,
	/// If the user has avatar cloning on
	pub allow_avatar_copying: bool,
	#[serde(rename = "date_joined")]
	// TODO: use time::Date
	/// When the user joined VRC
	pub date_joined: String,
	/// Notes about the user
	pub note: String,
}

/// 2FA variants
#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	PartialEq,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
)]
#[serde(rename_all = "camelCase")]
pub enum AdditionalAuthFactor {
	/// Email code
	EmailOtp,
	/// Authenticator app
	Totp,
	/// Recovery code
	Otp,
}

/// Response from the API when logging in
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
	/// If the login still requires 2FA
	#[serde(rename = "requiresTwoFactorAuth")]
	pub requires_additional_auth: Vec<AdditionalAuthFactor>,
}

/// Possible response types from the current user endpoint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LoginResponseOrCurrentUser {
	/// Information about the currently authenticated user
	User(Box<CurrentUser>),
	/// Details about the login, like needing additional 2FA verification
	Login(LoginResponse),
}

/// Returned if an error happens with authentication
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AuthenticationError {
	/// If the 2FA code was okay
	pub verified: bool,
}

/// Status of current authentication token
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AuthStatus {
	/// If the authentication is OK
	pub ok: bool,
	/// The token that the authentication is using
	pub token: String,
}

impl std::fmt::Debug for AuthStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("AuthStatus")
			.field("ok", &self.ok)
			.field("token", &"*****")
			.finish()
	}
}

/// Status for if the sent 2FA code was okay
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SecondFactorVerificationStatus {
	/// If the 2FA code was okay
	pub verified: bool,
}
