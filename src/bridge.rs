use std::str::FromStr;
use std::thread;
use std::time::Duration;

use regex::Regex;

use hyper::Client;
use hyper::client::Body;
use hyper::client::response::Response;

use rustc_serialize::{Encodable, Decodable};
use rustc_serialize::json::{self, Json};

use errors::{HueError, AppError};

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
    pub fn on() -> Self {
        CommandLight { on: Some(true), ..Default::default() }
    }
    /// Returns a `CommandLight` that turns a light on
    pub fn off() -> Self {
        CommandLight { on: Some(false), ..Default::default() }
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
struct RegisterResponse<T: Encodable + Decodable>{
    success: Option<T>,
    error: Option<AppError>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct User{
    username: String
}
/// A builder object for a `Bridge`
#[derive(Debug, Clone)]
pub struct BridgeBuilder{
    ip: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Discovery{
    id: String,
    internalipaddress: String
}

impl BridgeBuilder{
    /// Discovers the bridge via https://www.meethue.com/api/nupnp
    pub fn discover() -> Result<Self, HueError> {
        let client = Client::new();

        let mut res = try!(client.get("https://www.meethue.com/api/nupnp").send());

        let discoveries = try!(<Vec<Discovery>>::decode(&mut json::Decoder::new(try!(json::Json::from_reader(&mut res)))));

        Ok(
            BridgeBuilder {
                ip: discoveries[0].internalipaddress.clone()
            }
        )
    }
    /// Returns a `Bridge` from an already existing user
    pub fn from_username(self, username: String) -> Bridge {
        let BridgeBuilder{ip} = self;
        Bridge {
            username: username,
            ip: ip
        }
    }
    fn register(&self, devicetype: &str) -> Result<User, HueError>{
        let body = format!("{{devicetype: \"{}\"}}", devicetype);
        let body = body.as_bytes();
        let client = Client::new();
        let url = format!("http://{}/api", self.ip);
        let mut resp = try!(client.post(&url)
            .body(Body::BufBody(body, body.len()))
            .send());

        let rur = try!(RegisterResponse::decode(&mut json::Decoder::new(try!(Json::from_reader(&mut resp)))));

        if let Some(user) = rur.success{
            Ok(user)
        }else if let Some(error) = rur.error{
            Err(HueError::BridgeError(error))
        }else{
            Err(HueError::ProtocolError("Unrecognisable response".to_owned()))
        }
    }
    /// Registers a new user on the bridge
    pub fn register_user(self, devicetype: &str) -> Result<Bridge, HueError>{
        loop {
            match self.register(devicetype) {
                Ok(User{username}) => {
                    return Ok(Bridge{
                        username: username,
                        ip: self.ip.clone()
                    });
                }
                Err(HueError::BridgeError(ref error)) if error.code == 101 => {
                    println!("Push the bridge button");
                    thread::sleep(Duration::from_secs(5));
                }
                Err(e) => return Err(e)
            }
        }
    }
}

#[derive(Debug, Clone)]
/// The bridge connection
pub struct Bridge {
    ip: String,
    username: String,
}

impl Bridge {
    pub fn get_all_lights(&self) -> Result<Vec<IdentifiedLight>, HueError> {
        let url = format!("http://{}/api/{}/lights",
                          self.ip,
                          self.username);
        let client = Client::new();
        let mut resp = try!(client.get(&url[..]).send());
        let json = try!(json::Json::from_reader(&mut resp));
        let json_object = try!(json.as_object().ok_or(HueError::ProtocolError("malformed bridge response".to_string())));
        let mut lights: Vec<IdentifiedLight> = try!(json_object.iter()
            .map(|(k, v)| -> Result<IdentifiedLight, HueError> {
                let id: usize = try!(usize::from_str(k));
                let mut decoder = json::Decoder::new(v.clone());
                let light = try!(<Light as Decodable>::decode(&mut decoder));
                Ok(IdentifiedLight {
                    id: id,
                    light: light,
                })
            })
            .collect());
        lights.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(lights)
    }

    pub fn set_light_state(&self, light: usize, command: CommandLight) -> Result<Json, HueError> {
        let url = format!("http://{}/api/{}/lights/{}/state",
                          self.ip,
                          self.username,
                          light);
        let body = try!(json::encode(&command));
        let re1 = Regex::new("\"[a-z]*\":null").unwrap();
        let cleaned1 = re1.replace_all(&body, "");
        let re2 = Regex::new(",+").unwrap();
        let cleaned2 = re2.replace_all(&cleaned1, ",");
        let re3 = Regex::new(",\\}").unwrap();
        let cleaned3 = re3.replace_all(&cleaned2, "}");
        let re3 = Regex::new("\\{,").unwrap();
        let cleaned4 = re3.replace_all(&cleaned3, "{");
        let client = Client::new();
        let mut resp = try!(client.put(&url[..])
            .body(Body::BufBody(cleaned4.as_bytes(), cleaned4.as_bytes().len()))
            .send());
        Json::from_reader(&mut resp).map_err(From::from)
    }
}
