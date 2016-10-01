use hyper;
use std::convert::From;
use std::error::Error;
use rustc_serialize::json;
use std::num::ParseIntError;

use ::hue::Error as AppError;

#[derive(Debug)]
// TODO FIXME
/// Errors that can occur in this crate
pub enum HueError {
    ProtocolError(String),
    BridgeError(::hue::Error),
    EncoderError(json::EncoderError),
    DecoderError(json::DecoderError),
    ParserError(json::ParserError),
    HyperError(hyper::Error),
    ParseIntError(ParseIntError)
}

impl HueError {
    pub fn wrap<O>(a: &str) -> ::std::result::Result<O, HueError> {
        Err(HueError::ProtocolError(a.to_string()))
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
