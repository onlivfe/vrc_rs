use racal::Queryable;
use serde::{Deserialize, Serialize};

use super::Authentication;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// Also works as the login request
pub struct Instance {
	/// The world instance ID to get
	pub id: crate::id::WorldInstance,
}

impl Queryable<Authentication, crate::model::Instance> for Instance {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/instances/{}", crate::API_BASE_URI, self.id)
	}
}
