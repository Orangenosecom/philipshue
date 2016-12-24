#![warn(missing_docs)]
#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

//! Crate for communicating with the hue API

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "ssdp")]
extern crate ssdp;

extern crate serde;
extern crate serde_json;
extern crate hyper;
#[macro_use]
extern crate error_chain;

pub use bridge::Bridge;
#[cfg(feature = "nupnp")]
pub use bridge::discover;
#[cfg(feature = "upnp")]
pub use bridge::discover_upnp;
pub use hue::LightCommand;

/// Errors that can occur in the crate
pub mod errors;
/// Handles all the communication with the bridge
pub mod bridge;
/// Structs mapping the different JSON-objects used with Hue API
pub mod hue;
mod json;
