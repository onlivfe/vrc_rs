use vrc::{model::LoginResponseOrCurrentUser, query::Authentication};

mod common;

#[tokio::test]
#[ignore]
async fn current_user() -> Result<(), vrc::api_client::ApiError> {
	let api_client = common::api_client();

	let query = vrc::query::GetCurrentUser;
	let current_user = api_client.query::<LoginResponseOrCurrentUser, Authentication, vrc::query::GetCurrentUser>(query).await?;

	dbg!(&current_user);

	let current_user = match current_user {
		LoginResponseOrCurrentUser::User(user) => user,
		LoginResponseOrCurrentUser::Login(r) => {
			panic!("Expected to get current user details, but got login resp: {r:?}")
		}
	};

	assert!(!current_user.bio.is_empty());

	Ok(())
}
