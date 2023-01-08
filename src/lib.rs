#![doc(html_logo_url = "https://github.com/onlivfe/vrc_rs/raw/logo.png")]
//! Typed models for VRChat's API.
//!
//! This is fully work in progress, and published to be able to already setup the dependencies for [onlivfe](https://onlivfe.com).
//!
//! Please see [vrchatapi](https://docs.rs/vrchatapi) for a mature implementation.
//! It does though for example represents many things as `String`s or `Option`s
//! where an `enum` would be more appropriate. This implementation will differ
//! in that it'll try to align more with rust idioms. It's understandable though
//! for the Openapi generated code to not align that closely with them.

pub mod id;
