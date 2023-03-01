use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::{Authenticating, Authentication};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// Also works as the login request
pub struct GetCurrentUser;

impl Queryable<Authenticating, crate::model::LoginResponseOrCurrentUser>
	for GetCurrentUser
{
	fn url(&self, _: &Authenticating) -> String {
		format!("{}/auth/user", crate::API_BASE_URI)
	}
}

impl Queryable<Authentication, crate::model::LoginResponseOrCurrentUser>
	for GetCurrentUser
{
	fn url(&self, _: &Authentication) -> String {
		format!("{}/auth/user", crate::API_BASE_URI)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// Completes authentication with a 2FA code
pub enum VerifySecondFactor {
	/// A normal 2FA
	Code(String),
	/// A recovery code
	Recovery(String),
	/// An email code
	Email(String),
}

impl Queryable<Authentication, crate::model::SecondFactorVerificationStatus>
	for VerifySecondFactor
{
	fn url(&self, _: &Authentication) -> String {
		format!(
			"{}/auth/twofactorauth/{}/verify",
			crate::API_BASE_URI,
			match self {
				Self::Code(_) => "totp",
				Self::Recovery(_) => "otp",
				Self::Email(_) => "emailotp",
			}
		)
	}

	fn body(
		&self, _state: &Authentication,
	) -> Option<serde_json::Result<Vec<u8>>> {
		let code = match self {
			Self::Code(v) | Self::Recovery(v) | Self::Email(v) => v,
		};
		Some(serde_json::to_vec(&serde_json::json!({ "code": code })))
	}

	fn method(&self, _state: &Authentication) -> racal::RequestMethod {
		racal::RequestMethod::Post
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// Verifies that the current auth token is valid
pub struct Verify;

impl Queryable<Authentication, crate::model::AuthStatus> for Verify {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/auth", crate::API_BASE_URI)
	}
}
