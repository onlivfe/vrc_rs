use vrc::api_client::{ApiClient, ApiError};

mod common;

#[tokio::test]
#[ignore]
async fn user_tupper() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let query = vrc::query::User {
		id: "usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469".try_into().unwrap(),
	};
	let user: vrc::model::User = api_client.query(query).await?;

	dbg!(&user);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn user_system() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let query = vrc::query::User { id: "vF8OwsCETo".try_into().unwrap() };
	let user: vrc::model::User = api_client.query(query).await?;

	dbg!(&user);
	//assert!(!user.base.is_friend);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn user_self() -> Result<(), ApiError> {
	let friend_id = match &common::TEST_CONFIG.self_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client();

	let query = vrc::query::User { id: friend_id.to_owned() };
	let user: vrc::model::User = api_client.query(query).await?;

	dbg!(&user);
	//assert!(user.base.is_friend);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn user_friend() -> Result<(), ApiError> {
	let friend_id = match &common::TEST_CONFIG.friend_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client();

	let query = vrc::query::User { id: friend_id.to_owned() };
	let user: vrc::model::User = api_client.query(query).await?;

	dbg!(&user);
	//assert!(user.base.is_friend);

	Ok(())
}
