//! Wrappers for VRC IDs.
//!
//! Wrapping them IDs in newtypes makes sure you aren't trying to accidentally
//! compare different types of VRC IDs with each other like so:
//!
//! ```compile_fail,E0308
//! let user_id = vrc::id::User::from("usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469");
//! let instance_id = vrc::id::Instance::from("whatever-instance-ids-look-like");
//! assert!(user_id != record_id, "can't compare different types of IDs")
//! ```

use serde::{Deserialize, Serialize};

// TODO: serialization & deserilization customizations

/// A VRC user's ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum User {
	/// A modern user ID
	Standard(String),
	/// An old user ID without a prefix, less than 11 chars
	Legacy(String),
}

/// A VRC instance's ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Instance(String);

/// A VRC world's ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct World(String);

/// Offline or the id of the world or whatever type T is
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OfflineOr<T> {
	// TODO: Mangle serde enough to match these properly
	/// The ID was replaced by offline
	Offline,
	/// There exists an ID
	Id(T),
}

/// Offline or private or the id of the instance or whatever type T is
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OfflineOrPrivateOr<T> {
	// TODO: Mangle serde enough to match these properly
	/// The ID was replaced by offline
	Offline,
	/// There exists an ID
	Id(T),
}
