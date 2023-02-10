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

pub mod model;
