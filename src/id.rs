//! Wrappers for VRChat IDs.
//!
//! Wrapping them IDs in newtypes makes sure you aren't trying to accidentally
//! compare different types of VRChat IDs with each other like so:
//!
//! ```compile_fail,E0308
//! let user_id = vrc::id::User::from("usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469");
//! let instance_id = vrc::id::Instance::from("whatever-instance-ids-look-like");
//! assert!(user_id != record_id, "can't compare different types of IDs")
//! ```

/// A VRChat user's ID
pub enum User {
	Standard(String),
	Legacy(String),
}

/// A VRChat instance's ID
pub struct InstanceId(String);

/// A VRChat world's ID
pub struct WorldId(String);
