use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

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
	/// Also known as gray
	Offline,
}

impl Default for UserStatus {
	fn default() -> Self { Self::Offline }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a VRC user
pub struct User {
	/// If the user has avatar cloning on
	pub allow_avatar_copying: bool,
	/// Text that the user has written about themselves
	pub bio: String,
	/// Links that the user has added about themselves
	pub bio_links: Vec<String>,
	/// The avatar's image, for profile picture see the [profile pic
	/// override](Self::profile_pic_override)
	pub current_avatar_image_url: String,
	/// The avatar's smaller image, for profile picture see the [profile pic
	/// override](Self::profile_pic_override)
	pub current_avatar_thumbnail_image_url: String,
	#[serde(rename = "date_joined")]
	// TODO: use time::Date
	/// When the user joined VRC
	pub date_joined: String,
	/// If the user has some sort of a special status
	pub developer_type: DeveloperType,
	/// A users visual display name. This is what shows up in-game, and can
	/// different from their `username`. Changing display name is restricted to
	/// a cool down period.
	pub display_name: String,
	/// TODO: Figure out
	pub friend_key: String,
	/// The user's ID
	pub id: crate::id::User,
	/// If the user is a friend of the currently authenticated user
	pub is_friend: bool,
	/// Either a date-time or empty string.
	#[serde(rename = "last_activity", with = "rfc3339")]
	pub last_activity: OffsetDateTime,
	/// Either a date-time or empty string.
	#[serde(rename = "last_login", with = "rfc3339")]
	pub last_login: OffsetDateTime,
	// TODO: Platform enum
	/// This can be `standalonewindows` or `android`, but can also pretty much
	/// be any random Unity version or even `unknownplatform`.
	#[serde(rename = "last_platform")]
	pub last_platform: String,
	/// Possible profile picture URL
	pub profile_pic_override: String,
	/// If the user is online or not
	pub state: UserState,
	/// The status of the user
	pub status: UserStatus,
	/// User set status message
	pub status_description: String,
	/// Tags of the user
	pub tags: Vec<String>,
	/// URL to the user's icon, can be an empty string
	#[serde(default)]
	pub user_icon: String,
	/// Only returned if for current user. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429).
	#[serde(default)]
	pub username: Option<String>,
}

/// 2FA variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
	#[serde(rename = "requiresTwoFactorAuth")]
	requires_additional_auth: Vec<AdditionalAuthFactor>,
}

/// Possible response types from the current user endpoint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LoginResponseOrCurrentUser {
	/// Information about the currently authenticated user
	User(Box<User>),
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
