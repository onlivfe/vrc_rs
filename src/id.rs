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

macro_rules! add_id {
	(
		$(#[$meta:meta])*
		$name:ident
	) => {
		#[doc = concat!("An ID of a VRC ", stringify!($name))]
		///
		/// # Example usage
		///
		/// ```
		#[doc = concat!("use vrc::id::", stringify!($name), ";")]
		#[doc = concat!("let id1 = ", stringify!($name), "::try_from(\"totally-legit-id\").unwrap();")]
		#[doc = concat!("let id2 = ", stringify!($name), "::try_from(\"other-legit-id\").unwrap();")]
		/// assert!(id1 != id2);
		/// ```
		#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
		#[repr(transparent)]
		$(#[$meta])*
		pub struct $name(String);

		impl AsRef<str> for $name {
			/// Extracts a string slice containing the entire inner String.
			#[must_use]
			fn as_ref(&self) -> &str {
				&self.0
			}
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}

		impl TryFrom<String> for $name {
			type Error = &'static str;
			fn try_from(v: String) -> Result<Self, Self::Error> {
				Ok($name(v))
			}
		}

		/// For easier scripting, should use String otherwise.
		impl TryFrom<&'static str> for $name {
			type Error = &'static str;
			fn try_from(v: &'static str) -> Result<Self, Self::Error> {
				Self::try_from(v.to_owned())
			}
		}

		impl From<$name> for String {
			fn from(id: $name) -> String {
				id.0
			}
		}

		impl From<$name> for Any {
			fn from(id: $name) -> Any {
				Any::$name(id)
			}
		}
	};
}

add_id!(Avatar);
add_id!(User);
add_id!(Instance);
add_id!(World);

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
	/// Offline currently
	Private,
	// TODO: Mangle serde enough to match these properly
	/// The ID was replaced by offline
	Offline,
	/// There exists an ID
	Id(T),
}

/// Any of the VRC IDs
///
/// # Example usage
///
/// ```
/// let id1 = vrc::id::User::try_from("totally-legit-id").unwrap();
/// let id1: vrc::id::Any = id1.into();
/// let id2 = vrc::id::Instance::try_from("totally-legit-id").unwrap();
/// let id2: vrc::id::Any = id2.into();
/// assert!(id1 != id2);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Any {
	/// An avatar ID
	Avatar(Avatar),
	/// An user ID
	User(User),
	/// An instance ID
	Instance(Instance),
	/// A world ID
	World(World),
}

impl AsRef<str> for Any {
	/// Extracts a string slice containing the entire inner String.
	#[must_use]
	fn as_ref(&self) -> &str {
		match self {
			Self::Avatar(v) => v.as_ref(),
			Self::User(v) => v.as_ref(),
			Self::Instance(v) => v.as_ref(),
			Self::World(v) => v.as_ref(),
		}
	}
}

impl std::fmt::Display for Any {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}
