#![cfg(feature = "api_client")]
// Something's funky with checking if these are used or not.
#![allow(dead_code)]

use once_cell::sync::Lazy;
use vrc::{api_client::AuthenticatedVRC, query::Authentication};

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	") - tests",
);

#[derive(Default, serde::Deserialize)]
pub struct TestConfig {
	pub friend_id: Option<vrc::id::User>,
	pub self_id: Option<vrc::id::User>,
}

pub static TEST_CONFIG: Lazy<TestConfig> = Lazy::new(|| {
	std::fs::read("test-config.json")
		.map(|bytes| serde_json::from_slice::<TestConfig>(&bytes).ok())
		.ok()
		.flatten()
		.unwrap_or_default()
});

pub static USER_AUTH: Lazy<Authentication> = Lazy::new(|| {
	let user_auth: Authentication = serde_json::from_slice::<Authentication>(
		&std::fs::read("user-auth.json").expect(
			"must have a prepared `user-auth.json` file for live API testing",
		),
	)
	.expect("`user-auth.json` file to parse into user auth");

	assert!(!user_auth.token.is_empty());
	assert!(user_auth.token.len() > 20);

	user_auth
});

pub fn api_client() -> AuthenticatedVRC {
	AuthenticatedVRC::new(USER_AGENT.to_owned(), USER_AUTH.clone()).unwrap()
}
