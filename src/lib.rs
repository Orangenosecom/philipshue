#![warn(missing_docs)]

//! Crate for communicating with the hue API

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod clean{
    use regex::Regex;
    lazy_static!{
        static ref REMOVE_NULL: Regex = Regex::new("\"[a-z]*\":null,?").unwrap();
        static ref TRAILING_COMMA: Regex = Regex::new(",\\}").unwrap();
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
