use vrc::api_client::{ApiClient, ApiError};

mod common;

#[tokio::test]
#[ignore]
async fn friends() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let query = vrc::query::ListFriends;
	let friends: Vec<vrc::model::Friend> = api_client.query(query).await?;

	dbg!(&friends);
	// Sorry if you've got no friends x'P
	assert!(!friends.is_empty());
	let first_friend = friends.first().unwrap();
	assert!(first_friend.base.is_friend);
	assert!(!first_friend.base.id.as_ref().is_empty());
	// Yeah the first friend needs status for this test...
	// Sorry if your friends don't set statuses and run this x'P
	assert!(!first_friend.base.status_description.is_empty());

	Ok(())
}
