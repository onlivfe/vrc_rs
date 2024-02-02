use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::Authentication;

/// Gets information about a specific group
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Group {
	/// The ID of the group to get information about
	pub id: crate::id::Group,
}

impl Queryable<Authentication, crate::model::Group> for Group {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/groups/{}", crate::API_BASE_URI, self.id.as_ref())
	}
}

/// Gets audit logs from a specific group
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GroupAuditLogs {
	/// The ID of the group to get audit logs from
	pub id: crate::id::Group,
	/// The count of how many logs to get (1 - 100)
	pub n: Option<u8>,
	/// The offset of how many logs to get
	pub offset: Option<usize>,
	// TODO: startDate & endDate
}

impl Queryable<Authentication, crate::model::GroupAuditLogs>
	for GroupAuditLogs
{
	fn url(&self, _: &Authentication) -> String {
		let mut query =
			format!("{}/groups/{}/auditLogs?", crate::API_BASE_URI, self.id.as_ref());

		if let Some(n) = self.n {
			let param = format!("&n={n}");
			query.push_str(&param);
		}

		if let Some(offset) = self.offset {
			let param = format!("&offset={offset}");
			query.push_str(&param);
		}

		query
	}
}
