#![warn(missing_docs)]

//! Crate for communicating via the hue API

extern crate rustc_serialize;
extern crate hyper;
extern crate regex;

/// All things errors
pub mod errors;
/// Module responsible for communicating with the Hue bridge
pub mod bridge;
