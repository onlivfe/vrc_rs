#![cfg(feature = "api_client")]

use std::default;

use vrc::{
	api_client::{ApiClient, ApiError},
	model::{Group, GroupAuditLogs},
};

mod common;

#[tokio::test]
#[ignore]
async fn group() -> Result<(), ApiError> {
	let group_id = match &common::TEST_CONFIG.group_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client()?;

	let query = vrc::query::Group { id: group_id.clone() };
	let group: Group = api_client.query(query).await?;

	dbg!(&group);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn group_audit_logs() -> Result<(), ApiError> {
	let group_id = match &common::TEST_CONFIG.moderating_group_id {
		Some(v) => v,
		None => return Ok(()),
	};

	let api_client = common::api_client()?;

	let query =
		vrc::query::GroupAuditLogs { id: group_id.clone(), n: None, offset: None };
	let audit_logs: GroupAuditLogs = api_client.query(query).await?;

	dbg!(&audit_logs);

	Ok(())
}
