//! Avatars & worlds

use serde::{Deserialize, Serialize};

/// Information about a Unity package
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityPackage {
	// TODO: Add all fields
	/// The ID of the package
	pub id: crate::id::UnityPackage,
}

/// Information about a VRC world
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
	// TODO: Add all fields
	/// The ID of the avatar
	pub id: crate::id::Avatar,
}

/// Information about a VRC avatar
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
	// TODO: Add all fields
	/// The ID of the world
	pub id: crate::id::World,
}
