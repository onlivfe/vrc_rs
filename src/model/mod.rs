//! Models of the API responses to queries.

// Can't do much about API types
#![allow(clippy::struct_excessive_bools)]

mod avatars;
pub use avatars::*;
mod friends;
pub use friends::*;
mod instances;
pub use instances::*;
mod notifications;
pub use notifications::*;
mod users;
pub use users::*;
mod tags;
pub use tags::*;
