use hyper;
use std::convert::From;
use rustc_serialize::json;
use std::num::ParseIntError;

#[derive(Debug)]
// TODO FIXME
/// Errors that can occur in this crate
pub enum HueError {
    /// A general protocol error
    ProtocolError(String),
    /// An error that occured in the bridge
    BridgeError(::hue::Error),
    /// A `json::EncoderError`
    EncoderError(json::EncoderError),
    /// A `json::DecoderError`
    DecoderError(json::DecoderError),
    /// A `json::ParserError`
    ParserError(json::ParserError),
    /// A `hyper::Error`
    HyperError(hyper::Error),
    /// An `std::num::ParseIntError`
    ParseIntError(ParseIntError)
}

impl HueError {
    /// Returns a `ProtocolError` with the given string
    pub fn wrap<S: ToString, O>(s: S) -> ::std::result::Result<O, HueError> {
        Err(HueError::ProtocolError(s.to_string()))
    }
}

impl From<json::EncoderError> for HueError {
    fn from(err: json::EncoderError) -> HueError {
        HueError::EncoderError(err)
    }
}

impl From<json::DecoderError> for HueError {
    fn from(err: json::DecoderError) -> HueError {
        HueError::DecoderError(err)
    }
}

impl From<json::ParserError> for HueError {
    fn from(err: json::ParserError) -> HueError {
        HueError::ParserError(err)
    }
}

impl From<hyper::error::Error> for HueError {
    fn from(err: hyper::error::Error) -> HueError {
        HueError::HyperError(err)
    }
}

impl From<ParseIntError> for HueError {
    fn from(err: ParseIntError) -> HueError {
        HueError::ParseIntError(err)
    }
}
