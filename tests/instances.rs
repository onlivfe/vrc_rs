#![cfg(feature = "api_client")]

use vrc::{
	api_client::{ApiClient, ApiError},
	model::{Instance, World, WorldListing},
};

mod common;

#[tokio::test]
#[ignore]
async fn active_instance() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let mut query = vrc::query::ActiveWorlds::default();
	query.pagination.limit = 1;
	let active_worlds: Vec<WorldListing> = api_client.query(query).await?;
	let world = active_worlds.first().expect("there to be an active world");
	let query = vrc::query::World { id: world.base.id.clone() };
	let world: World = api_client.query(query).await?;
	let instance =
		world.instances.first().expect("there to be an active instance");
	let id = vrc::id::WorldInstance {
		world: world.base.id.clone(),
		instance: instance.0.clone(),
	};
	let query = vrc::query::Instance { id };
	let instance: Instance = dbg!(api_client.query(query).await?);

	assert!(instance.active);
	assert!(
		[vrc::model::InstancePrivacy::Public, vrc::model::InstancePrivacy::Group]
			.contains(&instance.privacy)
	);
	//assert!(instance.can_request_invite);
	assert!(instance.capacity > 0);

	Ok(())
}
