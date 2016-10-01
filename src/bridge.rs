use std::str::FromStr;

use regex::Regex;

use hyper::Client;
use hyper::client::Body;
use hyper::client::response::Response;

use rustc_serialize::{Encodable, Decodable};
use rustc_serialize::json::{self, Json};

use errors::HueError;
use ::hue::*;

/// Attempts to discover bridges using `https://www.meethue.com/api/nupnp`
pub fn discover() -> Result<Vec<Discovery>, HueError> {
    let client = Client::new();

    let mut res = try!(client.get("https://www.meethue.com/api/nupnp").send());

    <Vec<Discovery>>::decode(&mut json::Decoder::new(try!(json::Json::from_reader(&mut res))))
    .map_err(From::from)
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
/// Responses from the `discover` function
pub struct Discovery{
    id: String,
    internalipaddress: String
}

impl Discovery {
    /// Returns a `BridgeBuilder` with the ip of the bridge discovered
    pub fn build_bridge(self) -> BridgeBuilder{
        let Discovery{internalipaddress,..} = self;
        BridgeBuilder{
            ip: internalipaddress
        }
    }
    /// The ip of this discovered bridge
    pub fn ip(&self) -> &str{
        &self.internalipaddress
    }
    /// The id of this discovered bridge
    pub fn id(&self) -> &str{
        &self.id
    }
}

/// A builder object for a `Bridge`
#[derive(Debug)]
pub struct BridgeBuilder{
    ip: String
}

impl BridgeBuilder{
    pub fn from_ip(ip: String) -> Self{
        BridgeBuilder{
            ip: ip
        }
    }
    /// Returns a `Bridge` from an already existing user
    pub fn from_username(self, username: String) -> Bridge {
        let BridgeBuilder{ip} = self;
        Bridge {
            client: Client::new(),
            username: username,
            ip: ip
        }
    }
    /// Registers a new user on the bridge
    pub fn register_user(self, devicetype: &str) -> RegisterIter{
        RegisterIter(Some(self), devicetype)
    }
}

#[derive(Debug)]
/// Iterator that tries to register a new user each iteration
// TODO Better documention
pub struct RegisterIter<'a>(Option<BridgeBuilder>, &'a str);

impl<'a> Iterator for RegisterIter<'a> {
    type Item = Result<Bridge, HueError>;
    fn next(&mut self) -> Option<Self::Item>{
        if self.0.is_some(){
            let client = Client::new();
            let bb = ::std::mem::replace(&mut self.0, None).unwrap();

            let body = format!("{{\"devicetype\": {:?}}}", self.1);
            let body = body.as_bytes();
            let url = format!("http://{}/api", bb.ip);
            let mut resp = match client.post(&url)
                .body(Body::BufBody(body, body.len()))
                .send() {
                    Ok(r) => r,
                    Err(e) => return Some(Err(HueError::from(e)))
                };


            let rur = match Json::from_reader(&mut resp)
            .map_err(From::from)
            .and_then(|r| <Vec<HueResponse<User>>>::decode(&mut json::Decoder::new(r))) {
                Ok(mut r) => r.pop().unwrap(),
                Err(e) => return Some(Err(HueError::from(e)))
            };

            Some(if let Some(User{username}) = rur.success{
                let BridgeBuilder{ip} = bb;

                Ok(Bridge{
                    ip: ip,
                    client: client,
                    username: username
                })
            }else if let Some(error) = rur.error{
                self.0 = Some(bb);
                Err(HueError::BridgeError(error))
            }else{
                Err(HueError::ProtocolError("Unrecognisable response".to_owned()))
            })
        }else{
            None
        }
    }
}

#[derive(Debug)]
/// The bridge connection
pub struct Bridge {
    client: Client,
    ip: String,
    username: String,
}

impl Bridge {
    /// Gets all lights from the bridge
    // TODO Clean up
    pub fn get_all_lights(&self) -> Result<Vec<IdentifiedLight>, HueError> {
        let url = format!("http://{}/api/{}/lights",
                          self.ip,
                          self.username);

        let mut resp = try!(self.client.get(&url[..]).send());
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
    /// Sends a `CommandLight` to set the state of a light
    // TODO Clean up
    pub fn set_light_state(&self, light: usize, command: CommandLight) -> Result<Json, HueError> {
        let url = format!("http://{}/api/{}/lights/{}/state",
                          self.ip,
                          self.username,
                          light);
        let body = try!(json::encode(&command));
        let re1 = Regex::new("\"[a-z]*\":null,?").unwrap();
        let cleaned1 = re1.replace_all(&body, "");
        let re2 = Regex::new(",\\}").unwrap();
        let cleaned2 = re2.replace_all(&cleaned1, "}");
        let body = cleaned2.as_bytes();

        let mut resp = try!(self.client.put(&url)
            .body(Body::BufBody(body, body.len()))
            .send());
        let mut decoder = json::Decoder::new(try!(Json::from_reader(&mut resp)));
        HueResponse::decode(&mut decoder).map_err(From::from)
    }
}
