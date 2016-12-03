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
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub use bridge::{discover, Bridge};
pub use hue::LightCommand;

mod clean{
    use regex::Regex;
    lazy_static!{
        static ref REMOVE_NULL: Regex = Regex::new(r#""\w*":null,?"#).unwrap();
        static ref TRAILING_COMMA: Regex = Regex::new(r",\}").unwrap();
    }
    /// Removes null values from the JSON-formatted String
    pub fn clean_json(s: String) -> String{
        let cleaned = REMOVE_NULL.replace_all(&s, "");
        TRAILING_COMMA.replace_all(&cleaned, "}")
    }
}

/// Errors that can occur in the crate
pub mod errors;
/// Handles all the communication with the bridge
pub mod bridge;
/// Structs mapping the different JSON-objects used with Hue API
pub mod hue;
