#![cfg(feature = "api_client")]

use vrc::api_client::{ApiClient, ApiError};

mod common;

#[tokio::test]
#[ignore]
async fn user_tupper() -> Result<(), ApiError> {
	let api_client = common::api_client()?;

	let query = vrc::query::User {
		id: "usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469".parse().unwrap(),
	};
	let user: vrc::model::AnyUser = api_client.query(query).await?;

	dbg!(&user);

	let user = user.into_user();

	assert!(user.base.developer_type == vrc::model::DeveloperType::Internal);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn user_system() -> Result<(), ApiError> {
	let api_client = common::api_client()?;

	let query = vrc::query::User { id: "vF8OwsCETo".parse().unwrap() };
	let user: vrc::model::AnyUser = api_client.query(query).await?;

	dbg!(&user);

	let user = match user {
		vrc::model::AnyUser::Authenticated(_) => {
			panic!("didn't expect the system user to be self")
		}
		vrc::model::AnyUser::Friend(_) => {
			panic!("didn't expect the system user to be a friend")
		}
		vrc::model::AnyUser::User(u) => *u,
	};

	assert!(!user.base.is_friend);
	assert!(user.base.developer_type == vrc::model::DeveloperType::Internal);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn user_self() -> Result<(), ApiError> {
	let friend_id = match &common::TEST_CONFIG.self_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client()?;

	let query = vrc::query::User { id: friend_id.to_owned() };
	let user: vrc::model::AnyUser = api_client.query(query).await?;

	dbg!(&user);

	let user = match user {
		vrc::model::AnyUser::Authenticated(a) => *a,
		vrc::model::AnyUser::Friend(_) => {
			panic!("didn't expect self user to be only a friend")
		}
		vrc::model::AnyUser::User(_) => {
			panic!("didn't expect self user to only an user")
		}
	};

	assert!(user.account.email_verified);
	assert!(user.base.base.developer_type == vrc::model::DeveloperType::None);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn user_friend() -> Result<(), ApiError> {
	let friend_id = match &common::TEST_CONFIG.friend_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client()?;

	let query = vrc::query::User { id: friend_id.to_owned() };
	let user: vrc::model::AnyUser = api_client.query(query).await?;

	dbg!(&user);

	let friend = match user {
		vrc::model::AnyUser::Authenticated(_) => {
			panic!("didn't expect friend user to be self")
		}
		vrc::model::AnyUser::Friend(f) => *f,
		vrc::model::AnyUser::User(_) => {
			panic!("didn't expect friend user to only an user")
		}
	};

	assert!(friend.base.base.is_friend);
	assert!(!friend.friend.friend_key.is_empty());

	Ok(())
}
