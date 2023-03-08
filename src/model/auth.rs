use serde::{Deserialize, Serialize};

use super::CurrentAccount;

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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LoginResponseOrCurrentUser {
	/// Information about the currently authenticated user
	User(Box<CurrentAccount>),
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
