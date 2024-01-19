#![cfg(feature = "api_client")]

use vrc::api_client::{ApiClient, ApiError};

mod common;

#[tokio::test]
#[ignore]
async fn current_user() -> Result<(), ApiError> {
	let api_client = common::api_client()?;

	let query = vrc::query::GetCurrentUser;
	let current_user: vrc::model::CurrentAccount =
		api_client.query(query).await?;

	dbg!(&current_user);

	assert!(!current_user.base.bio.is_empty());
	assert!(!current_user.current.username.is_empty());

	Ok(())
}
