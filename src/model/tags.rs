use serde::{Deserialize, Serialize};

// TODO: Customize serde to match & remove prefixes
// TODO: Add enums for the tags
// Maybe can use https://docs.rs/serde_with/latest/serde_with/macro.with_prefix.html

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
/// A tag that users and worlds for example can have
pub enum Tag {
	/// Tags which started with `system_`, will have the prefix removed
	System(String),
	/// Tags which started with `admin_`, will have the prefix removed
	Admin(String),
	/// Tags which started with `language_`, will have the prefix removed
	Language(String),
	/// Tags which started with `author_`, will have the prefix removed
	Author(String),
	/// Unmatched tags, will be the full tags without any prefix stripping
	Other(String),
}
