#![cfg(feature = "api_client")]
// Something's funky with checking if these are used or not.
//#![allow(dead_code)]
//use vrc::{};

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	") - tests",
);

pub fn api_client() -> vrc::api_client::AuthenticatedVRC {
	todo!();
}
