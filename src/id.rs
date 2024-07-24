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

use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

// TODO: serialization & deserilization customizations

macro_rules! add_id {
	(
		$(#[$meta:meta])*
		$name:ident,
		$id_matches:expr
	) => {
		#[doc = concat!("An ID of a VRC ", stringify!($name))]
		///
		/// # Example usage
		///
		/// Note that parse checks that the ID follows the correct format:
		///
		/// ```
		#[doc = concat!("use vrc::id::", stringify!($name), ";")]
		#[doc = concat!("let parse_res = \"totally-legit-id\".parse::<", stringify!($name), ">();")]
		/// assert!(parse_res.is_err());
		/// ```
		///
		/// But parsing checks can also be ignored by using infallible `From<String>` implementations:
		///
		/// ```
		#[doc = concat!("use vrc::id::", stringify!($name), ";")]
		/// // Note that parsing will fail with the wrong format
		#[doc = concat!("let id1 = ", stringify!($name), "::from(\"totally-legit-id\");")]
		#[doc = concat!("let id2 = ", stringify!($name), "::from(\"other-totally-legit-id\");")]
		/// assert!(id1 != id2);
		/// ```
		///
		/// The deserialization also checks the that the IDs format is valid, whilst serialization does not check the validity.
		#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
		#[repr(transparent)]
		$(#[$meta])*
		pub struct $name(String);

		impl $name {
			/// Checks if the ID matches the expected format
			#[must_use]
			pub fn is_valid(&self) -> bool {
				$id_matches(&self.0)
			}
		}

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

		/// Fallible: "...".`parse::`<id::Type>()
		impl std::str::FromStr for $name {
			type Err = &'static str;

			fn from_str(id: &str) -> Result<Self, Self::Err> {
				if !$id_matches(&id) {
					return Err(concat!("ID doesn't match expected format"))
				}
				Ok(Self(id.to_owned()))
			}
		}

		/// Infallible: `id::Type::from`("...") or "...".`into()`
		impl From<&str> for $name {
			fn from(id: &str) -> Self {
				Self(id.to_owned())
			}
		}

		/// Infallible: `id::Type::from`(String) or String.`into()`
		impl From<String> for $name {
			fn from(id: String) -> Self {
				Self(id)
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

		/// The deserializer will give an error if the inner String doesn't start with
		/// the proper prefix.
		impl<'de> serde::de::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct IdVisitor;

				impl<'de> Visitor<'de> for IdVisitor {
					type Value = $name;

					fn expecting(
						&self, formatter: &mut std::fmt::Formatter,
					) -> std::fmt::Result {
						formatter
							.write_str(concat!("a string UD if", stringify!($name), ", that is of the correct format"))
					}

					fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
					where
						E: serde::de::Error,
					{
						if !$id_matches(&v) {
							return Err(serde::de::Error::invalid_value(
								serde::de::Unexpected::Str(v),
								&"not matching the ID format",
							));
						}

						Ok($name(v.to_string()))
					}
				}

				deserializer.deserialize_str(IdVisitor)
			}
		}
	};
}

// VRC still has some legacy IDs, thus allowing 10 char strings without prefix
add_id!(Avatar, |v: &str| v.starts_with("avtr_") || v.len() == 10);
add_id!(Group, |v: &str| v.starts_with("grp_"));
// TODO: Manual implementation that breaks down instance name, type, region, and
// so on.
add_id!(Instance, |v: &str| v.contains("~region("));
add_id!(UnityPackage, |v: &str| v.starts_with("unp_") || v.len() == 10);
add_id!(User, |v: &str| v.starts_with("usr_") || v.len() == 10);
add_id!(GroupMember, |v: &str| v.starts_with("gmem_"));
add_id!(World, |v: &str| v.starts_with("wrld_") || v.len() == 10);

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

impl<T> OfflineOr<T> {
	/// Gives the ID as an option instead
	pub const fn as_option(&self) -> Option<&T> {
		match &self {
			Self::Offline => None,
			Self::Id(id) => Some(id),
		}
	}
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

impl<T> OfflineOrPrivateOr<T> {
	/// Gives the ID as an option instead
	pub const fn as_option(&self) -> Option<&T> {
		match &self {
			Self::Offline | Self::Private => None,
			Self::Id(id) => Some(id),
		}
	}
}

/// Any of the VRC IDs
///
/// # Example usage
///
/// ```
/// let id1 = "usr_totally-legit-uuid".parse::<vrc::id::User>().unwrap();
/// let id1: vrc::id::Any = id1.into();
/// let id2 =
/// 	"0000~group(grp_totally-legit-uuid)~groupAccessType(public)~region(us)"
/// 		.parse::<vrc::id::Instance>()
/// 		.unwrap();
/// let id2: vrc::id::Any = id2.into();
/// assert!(id1 != id2);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Any {
	/// An avatar ID
	Avatar(Avatar),
	/// An group ID
	Group(Group),
	/// An instance ID
	Instance(Instance),
	/// An ID for an Unity package
	UnityPackage(UnityPackage),
	/// An user ID
	User(User),
	/// A world ID
	World(World),
	/// A group member ID
	GroupMember(GroupMember),
}

impl AsRef<str> for Any {
	/// Extracts a string slice containing the entire inner String.
	#[must_use]
	fn as_ref(&self) -> &str {
		match self {
			Self::Avatar(v) => v.as_ref(),
			Self::Group(v) => v.as_ref(),
			Self::Instance(v) => v.as_ref(),
			Self::UnityPackage(v) => v.as_ref(),
			Self::User(v) => v.as_ref(),
			Self::World(v) => v.as_ref(),
			Self::GroupMember(v) => v.as_ref(),
		}
	}
}

impl std::fmt::Display for Any {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

/// A world instance's ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldInstance {
	/// The instance ID
	pub instance: Instance,
	/// The world ID
	pub world: World,
}

impl fmt::Display for WorldInstance {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.world.as_ref(), self.instance.as_ref())
	}
}

impl Serialize for WorldInstance {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

/// The deserializer will give an error if the inner String doesn't start with
/// the proper prefix.
impl<'de> serde::de::Deserialize<'de> for WorldInstance {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct IdVisitor;

		impl<'de> Visitor<'de> for IdVisitor {
			type Value = WorldInstance;

			fn expecting(
				&self, formatter: &mut std::fmt::Formatter,
			) -> std::fmt::Result {
				formatter
					.write_str(concat!("a string, WorldInstance, that is of format `{vrc::id:World}:{vrc::id:Instance}`"))
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				let (world, instance) = v.split_once(':').ok_or_else(|| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Str(v),
						&"should be able to be split at a `:` character",
					)
				})?;

				let world: World = world.parse().map_err(|e| {
					serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &e)
				})?;
				let instance: Instance = instance.parse().map_err(|e| {
					serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &e)
				})?;

				Ok(WorldInstance { instance, world })
			}
		}

		deserializer.deserialize_str(IdVisitor)
	}
}

#[cfg(test)]
#[test]
fn user_id_parsing() {
	// Tupper
	let id = "\"usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469\"";
	assert!(serde_json::from_str::<crate::id::User>(id).is_ok());

	let id = "\"grp_c1644b5b-3ca4-45b4-97c6-a2a0de70d469\"";
	assert!(serde_json::from_str::<crate::id::User>(id).is_err());

	// Valid length old user ID
	let id = "\"qYZJsbJRqA\"";
	assert!(serde_json::from_str::<crate::id::User>(id).is_ok());

	// Invalid length
	let id = "\"qYZJsbJRqA1\"";
	assert!(serde_json::from_str::<crate::id::User>(id).is_err());
}

#[cfg(test)]
#[test]
fn world_and_instance() {
	let original_id = "\"wrld_ba913a96-fac4-4048-a062-9aa5db092812:12345~hidden(usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469)~region(eu)~nonce(27e8414a-59a0-4f3d-af1f-f27557eb49a2)\"";
	let id: crate::id::WorldInstance = serde_json::from_str(original_id)
		.expect("to be able to deserialize WorldInstance");
	let id: String =
		serde_json::to_string(&id).expect("to be able to serialize WorldInstance");
	assert_eq!(original_id, id);
}

#[cfg(test)]
#[test]
fn strict_from_string() {
	use std::str::FromStr;

	let original_id = "\"grp_93451756-8327-4ecc-b978-3e60aa9f64a9\"";
	let id: crate::id::Group =
		serde_json::from_str(original_id).expect("to be able to deserialize Group");
	let id: String =
		serde_json::to_string(&id).expect("to be able to serialize a valid Group");
	assert_eq!(original_id, id);

	let original_id = "\"93451756-8327-4ecc-b978-3e60aa9f64a9\"";

	// For now, deserialization is still allowed with invalid IDs
	//assert!(serde_json::from_str::<crate::id::Group>(original_id).is_err(),
	// "deserializing an invalid Group ID errors");
	assert!(
		crate::id::Group::from_str(original_id).is_err(),
		"from_str for an invalid Group ID errors"
	);
	// From<String> implementations are infallible, which should always should
	// work...
	let id = crate::id::Group::from(original_id);
	assert!(
		!id.is_valid(),
		"Force converted group ID can be detected as invalid"
	);
	let _id: String = serde_json::to_string(&id)
		.expect("to be able to serialize an invalid Group");
}
