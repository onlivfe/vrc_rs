#![cfg(feature = "api_client")]

use vrc::{
	api_client::{ApiClient, ApiError},
	model::{Group, GroupAuditLogs, GroupBan},
	query::Pagination,
};

mod common;

#[tokio::test]
#[ignore]
async fn group() -> Result<(), ApiError> {
	let group_id = match &common::TEST_CONFIG.group_id {
		Some(v) => v,
		None => {
			println!("Skipping test {} due to lack of group id", stringify!(group));
			return Ok(());
		}
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
		None => {
			println!(
				"Skipping test {} due to lack of moderating group id",
				stringify!(group_audit_logs)
			);
			return Ok(());
		}
	};

	let api_client = common::api_client()?;

	let query = vrc::query::GroupAuditLogs {
		id: group_id.clone(),
		pagination: Pagination::default(),
	};
	let audit_logs: GroupAuditLogs = api_client.query(query).await?;

	dbg!(&audit_logs);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn group_bans() -> Result<(), ApiError> {
	let group_id = match &common::TEST_CONFIG.moderating_group_id {
		Some(v) => v,
		None => {
			println!(
				"Skipping test {} due to lack of moderating group id",
				stringify!(group_bans)
			);
			return Ok(());
		}
	};

	let api_client = common::api_client()?;

	let query = vrc::query::GroupBans {
		id: group_id.clone(),
		pagination: Pagination::default(),
	};
	let group_bans: Vec<GroupBan> = api_client.query(query).await?;

	dbg!(&group_bans);

	assert!(!group_bans.is_empty());

	Ok(())
}
