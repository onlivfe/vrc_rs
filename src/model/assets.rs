//! Avatars & worlds

use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use time::{serde::rfc3339, OffsetDateTime};
use url::Url;

/// If a world has been released publicly for example
#[derive(
	Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr,
)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseStatus {
	/// Publicly released
	Public,
	/// Not released at all
	Private,
	/// Not findable but also not private
	Hidden,
}

/// Information about what platform the unity package supports
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityPackageSupports {
	/// The platform that the package is for (`android` / `standalonewindows`)
	pub platform: String,
	/// The unity version
	pub unity_version: String,
}

/// Information about an Unity package
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityPackage {
	/// The ID of the package
	pub id: crate::id::UnityPackage,
	#[serde(flatten)]
	/// Which unity versions the package supports
	pub supports: UnityPackageSupports,
}

/// Information about a VRC world
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldData {
	/// The ID of the world creator
	pub author_id: crate::id::User,
	/// The display name of the author
	pub author_name: String,
	/// How many players normally can fit into an instance of this world
	pub capacity: u16,
	/// When the world was initially uploaded
	#[serde(rename = "created_at", with = "rfc3339")]
	pub created_at: OffsetDateTime,
	/// How many times the world has been added to favorites
	pub favorites: u64,
	/// How trending the world is
	pub heat: u32,
	/// The ID of the world
	pub id: crate::id::World,
	/// An image for displaying the world
	pub image_url: Url,
	/// When the world was published to labs
	#[serde(
		default,
		deserialize_with = "crate::deserialize_optional_date",
		serialize_with = "rfc3339::option::serialize"
	)]
	pub labs_publication_date: Option<OffsetDateTime>,
	/// The name of the world
	pub name: String,
	/// How many users are in instances of the world
	pub occupants: u32,
	/// Seems to always be `vrchat`
	pub organization: String,
	/// How popular the world is
	pub popularity: u32,
	/// A `YouTube` ID that's supposed to be used for a preview of the world
	///
	/// Many creators seem to use random videos here though
	// If fails, need serde_with none on null
	#[serde(default)]
	pub preview_youtube_id: Option<String>,
	/// When the world was published
	#[serde(
		default,
		deserialize_with = "crate::deserialize_optional_date",
		serialize_with = "rfc3339::option::serialize"
	)]
	pub publication_date: Option<OffsetDateTime>,
	/// The release status of the world
	pub release_status: ReleaseStatus,
	/// The tags of the world
	pub tags: Vec<String>,
	/// A preview image of the world
	pub thumbnail_image_url: Url,
	/// When the world was last updated
	#[serde(rename = "updated_at", with = "rfc3339")]
	pub updated_at: OffsetDateTime,
	/// How many times the world has been visited total
	pub visits: u64,
}

/// Limited information about a world
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldListing {
	/// Base world data
	#[serde(flatten)]
	pub base: WorldData,
	/// Minimal listing of unity packages support
	pub unity_packages: Vec<UnityPackageSupports>,
}

/// Extended information about a world
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
	/// Base world data
	#[serde(flatten)]
	pub base: WorldData,
	/// If the world is featured or not
	pub featured: bool,
	/// List of instances
	pub instances: Vec<(crate::id::Instance, u16)>,
	/// How many users are in private sessions of the world
	pub private_occupants: u32,
	/// How many users are in public sessions of the world
	pub public_occupants: u32,
	/// Listing of unity packages support
	pub unity_packages: Vec<UnityPackage>,
	/// The incrementing version of the world
	pub version: u32,
}

/// Information about a VRC avatar
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
	// TODO: Model
	/// The ID of the avatar
	pub id: crate::id::Avatar,
}
