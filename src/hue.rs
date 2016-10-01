use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};
use rustc_serialize::json::{self, Json};

#[derive(Debug,Copy,Clone,RustcDecodable)]
pub struct LightState {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8,
    pub ct: Option<u16>,
}

#[derive(Debug,Clone,RustcDecodable)]
pub struct Light {
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub uniqueid: String,
    pub state: LightState,
}

#[derive(Debug,Clone)]
pub struct IdentifiedLight {
    pub id: usize,
    pub light: Light,
}

#[derive(Debug, Default, Clone, Copy, RustcEncodable, RustcDecodable)]
/// Struct for building a command that will be sent to the Hue bridge telling it what to do with a light
///
/// View [the lights-api documention](http://www.developers.meethue.com/documentation/lights-api) for more information
pub struct CommandLight {
    /// Whether to turn the light off or on
    pub on: Option<bool>,
    /// Brightness of the colour of the light
    pub bri: Option<u8>,
    /// The hue of the colour of the light
    pub hue: Option<u16>,
    /// The saturation of the colour of the light
    pub sat: Option<u8>,
    /// The Mired Color temperature of the light. 2012 connected lights are capable of 153 (6500K) to 500 (2000K).
    pub ct: Option<u16>,
}

impl CommandLight {
    /// Returns a `CommandLight` that turns a light on
    pub fn on(self) -> Self {
        CommandLight { on: Some(true), ..self }
    }
    /// Returns a `CommandLight` that turns a light on
    pub fn off(self) -> Self {
        CommandLight { on: Some(false), ..self }
    }
    /// Sets the brightness to set the light to
    pub fn with_bri(self, b: u8) -> Self {
        CommandLight { bri: Some(b), ..self }
    }
    /// Sets the hue to set the light to
    pub fn with_hue(self, h: u16) -> Self {
        CommandLight { hue: Some(h), ..self }
    }
    /// Sets the saturation to set the light to
    pub fn with_sat(self, s: u8) -> Self {
        CommandLight { sat: Some(s), ..self }
    }
    /// Sets the temperature to set the light to
    pub fn with_ct(self, c: u16) -> Self {
        CommandLight { ct: Some(c), ..self }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
/// A response that either is an error or a success
pub struct HueResponse<T: Encodable + Decodable>{
    /// The result from the bridge if it didn't fail
    pub success: Option<T>,
    /// The error that was returned from the bridge
    pub error: Option<Error>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
/// A user object returned from the API
pub struct User{
    /// The username of the user
    pub username: String
}

#[derive(Debug)]
/// An error object returned from the API
pub struct Error {
    pub address: String,
    pub description: String,
    pub code: u8,
}

impl Encodable for Error {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match *self {
            Error { address: ref p_address, description: ref p_description, code: p_code } => {
                encoder.emit_struct("Error", 0, |encoder| {
                    try!(encoder.emit_struct_field("address", 0, |encoder| p_address.encode(encoder)));
                    try!(encoder.emit_struct_field("description", 1, |encoder| p_description.encode(encoder)));
                    try!(encoder.emit_struct_field("type", 2, |encoder| p_code.encode(encoder)));
                    Ok(())
                })
            }
        }
    }
}

impl Decodable for Error {
    fn decode<S: Decoder>(decoder: &mut S) -> Result<Error, S::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(Error {
                address: try!(decoder.read_struct_field("address", 0, |decoder| Decodable::decode(decoder))),
                description: try!(decoder.read_struct_field("description", 1, |decoder| Decodable::decode(decoder))),
                code: try!(decoder.read_struct_field("type", 2, |decoder| Decodable::decode(decoder))),
            })
        })
    }
}
