#![cfg(feature = "api_client")]

use vrc::{
	api_client::{ApiClient, ApiError},
	model::{World, WorldListing},
};

mod common;

#[tokio::test]
#[ignore]
async fn active_worlds() -> Result<(), ApiError> {
	let api_client = common::api_client()?;

	let query = vrc::query::ActiveWorlds::default();
	let active_worlds: Vec<WorldListing> = api_client.query(query).await?;

	dbg!(&active_worlds);

	assert!(!active_worlds.is_empty());

	Ok(())
}

#[tokio::test]
#[ignore]
async fn world() -> Result<(), ApiError> {
	let world_id = match &common::TEST_CONFIG.world_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client()?;

	let query = vrc::query::World { id: world_id.clone() };
	let world: World = api_client.query(query).await?;

	dbg!(&world);

	Ok(())
}
