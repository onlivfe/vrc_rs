#![cfg(feature = "api_client")]
// Something's funky with checking if these are used or not.
//#![allow(dead_code)]
//use vrc::{};

use once_cell::sync::Lazy;
use vrc::{api_client::AuthenticatedVRC, query::Authentication};

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	") - tests",
);

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
	AuthenticatedVRC::new(
		USER_AGENT.to_owned(),
		Authentication::from(&USER_AUTH.clone()),
	)
	.unwrap()
}
