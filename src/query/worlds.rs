use racal::Queryable;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::{Authentication, Order, Pagination};
use crate::model::ReleaseStatus;

/// The sorting for a world search
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr,
)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
#[strum(serialize_all = "camelCase")]
pub enum WorldsSort {
	Popularity,
	Heat,
	Trust,
	Shuffle,
	Random,
	Favorites,
	ReportScore,
	ReportCount,
	PublicationDate,
	LabsPublicationDate,
	Created,
	#[strum(serialize = " _created_at")]
	#[serde(rename = " _created_at")]
	CreatedAt,
	Updated,
	#[strum(serialize = " _updated_at")]
	#[serde(rename = " _updated_at")]
	UpdatedAt,
	Order,
	Relevance,
	Magic,
	Name,
}

impl Default for WorldsSort {
	fn default() -> Self { Self::Heat }
}

#[derive(
	Default, Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
/// Lists active worlds based on the  provided criteria
pub struct ActiveWorlds {
	/// If to filter based on featured status
	pub featured: Option<bool>,
	/// The sorting of the search
	pub sort: WorldsSort,
	/// The ordering of the search
	pub order: Order,
	/// If to filter based on a search string
	pub search: Option<String>,
	/// If to filter based on a search tag
	pub tag: Option<String>,
	/// If to exclude results based on a search tag
	pub no_tag: Option<String>,
	/// If to filter based on the release status
	pub release_status: Option<ReleaseStatus>,
	/// If to filter based on the max unity version
	pub max_unity_version: Option<String>,
	/// If to filter based on the min unity version
	pub min_unity_version: Option<String>,
	/// `standalonewindows` or `android` for example
	pub platform: Option<String>,
	/// The pagination for the query
	#[serde(flatten)]
	pub pagination: Pagination,
}

impl Queryable<Authentication, Vec<crate::model::WorldListing>>
	for ActiveWorlds
{
	fn url(&self, _: &Authentication) -> String {
		let mut query = format!(
			"{}/worlds/active?{}&sort={}&order={}",
			crate::API_BASE_URI,
			self.pagination.to_query_str(),
			&self.sort.as_ref(),
			&self.order.as_ref(),
		);

		if let Some(release_status) = &self.release_status {
			query.push_str("&releaseStatus=");
			query.push_str(release_status.as_ref());
		}

		if let Some(featured) = &self.featured {
			query.push_str("&featured=");
			query.push_str(&featured.to_string());
		}

		if let Some(search) = &self.search {
			query.push_str("&search=");
			query.push_str(search);
		}
		if let Some(tag) = &self.tag {
			query.push_str("&tag=");
			query.push_str(tag);
		}
		if let Some(no_tag) = &self.no_tag {
			query.push_str("&notag=");
			query.push_str(no_tag);
		}

		if let Some(max_unity_version) = &self.max_unity_version {
			query.push_str("&maxUnityVersion=");
			query.push_str(max_unity_version);
		}

		if let Some(min_unity_version) = &self.min_unity_version {
			query.push_str("&minUnityVersion=");
			query.push_str(min_unity_version);
		}

		if let Some(platform) = &self.platform {
			query.push_str("&platform=");
			query.push_str(platform);
		}

		query
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// Also works as the login request
pub struct World {
	/// The ID of the world to get
	pub id: crate::id::World,
}

impl Queryable<Authentication, crate::model::World> for World {
	fn url(&self, _: &Authentication) -> String {
		format!("{}/worlds/{}", crate::API_BASE_URI, self.id.as_ref())
	}
}
