#![doc(html_logo_url = "https://github.com/onlivfe/vrc_rs/raw/main/logo.png")]
//! Typed models for VRChat's API.
//!
//! This is fully work in progress, and published to be able to already setup the dependencies for [onlivfe](https://onlivfe.com).
//!
//! Please see [vrchatapi](https://docs.rs/vrchatapi) for a mature implementation.
//! It does though for example represents many things as `String`s or `Option`s
//! where an `enum` would be more appropriate. It's understandable though
//! for the Openapi generated code to not align that closely with them.
//!
//! This implementation will differ in that it'll try to align more with
//! the `rust™` way. Meaning the code will rename things to be meaningful,
//! use enums for a lot of things, and manually implement some traits like
//! debug for sensitive values such as passwords.

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]
// time macro causes warnings :/
#![allow(clippy::redundant_pub_crate)]

pub mod id;
pub mod model;
pub mod query;

/// The version 1 VRC API
pub const API_BASE_URI: &str = "https://vrchat.com/api/1";
/// VRC API for some reason uses a static cookie key for requests
pub const API_KEY: &str = "JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26";

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[cfg(feature = "api_client")]
pub mod api_client;
