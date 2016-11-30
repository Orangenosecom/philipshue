use hyper::Client;
use hyper::client::Body;

use serde_json::{to_string, from_reader, Map, Value};

use errors::HueError;
use ::hue::*;

/// Attempt to discover bridges using `https://www.meethue.com/api/nupnp`
pub fn discover() -> Result<Vec<Discovery>, HueError> {
    Client::new()
        .get("https://www.meethue.com/api/nupnp")
        .send()
        .map_err(HueError::from)
        .and_then(|ref mut r| from_reader(r).map_err(From::from))
}

/// Tries to register a user, returning the username if successful
///
/// This usually returns a `HueError::BridgeError` saying the link button needs to be pressed.
/// Therefore it recommended to call this function in a loop:
/// ## Example
/// ```no_run
/// use philips_hue_client::errors::{HueError, BridgeError};
/// use philips_hue_client::bridge::{self, Bridge};
///
/// let mut bridge = None;
/// // Discover a bridge
/// let bridge_ip = philips_hue_client::bridge::discover().unwrap().pop().unwrap().into_ip();
/// let devicetype = "my_hue_app#homepc";
///
/// // Keep trying to register a user
/// loop{
///     match bridge::register_user(&bridge_ip, devicetype){
///         // A new user has succesfully been registered and the username is returned
///         Ok(username) => {
///             bridge = Some(Bridge::new(bridge_ip, username));
///             break;
///         },
///         // Prompt the user to press the link button
///         Err(HueError::BridgeError{error: BridgeError::LinkButtonNotPressed, ..}) => {
///             println!("Please, press the link on the bridge. Retrying in 5 seconds");
///             std::thread::sleep(std::time::Duration::from_secs(5));
///         },
///         // Some other error happened
///         Err(e) => {
///             println!("Unexpected error occured: {:?}", e);
///             break
///         }
///     }
/// }
/// ```
pub fn register_user(ip: &str, devicetype: &str) -> Result<String, HueError>{
    let client = Client::new();

    let body = format!("{{\"devicetype\": {:?}}}", devicetype);
    let body = body.as_bytes();
    let url = format!("http://{}/api", ip);
    let mut resp = try!(client.post(&url)
        .body(Body::BufBody(body, body.len()))
        .send());

    let rur = try!(from_reader::<_, Vec<HueResponse<User>>>(&mut resp)).pop().unwrap();

    if let Some(User{username}) = rur.success{
        Ok(username)
    }else if let Some(error) = rur.error{
        Err(error.into())
    }else{
        Err(HueError::MalformedResponse)
    }
}

#[derive(Debug, Clone)]
/// A light with an ID
pub struct IdentifiedLight {
    /// The ID number of this light
    pub id: usize,
    /// The light object
    pub light: Light,
}

impl IdentifiedLight {
    /// Sets the state of a light by sending a `LightCommand` to the bridge for this light
    pub fn set_state(&mut self, bridge: &Bridge, command: LightCommand) -> Result<Vec<HueResponse<Value>>, HueError>{
        let url = format!("http://{}/api/{}/lights/{}/state",
                          bridge.ip,
                          bridge.username,
                          self.id);
        let body = ::clean::clean_json(try!(to_string(&command)));
        let body = body.as_bytes();

        let resps: Vec<HueResponse<Value>> = try!(bridge.client
            .put(&url)
            .body(Body::BufBody(body, body.len()))
            .send()
            .map_err(HueError::from)
            .and_then(|ref mut resp| from_reader(resp).map_err(From::from)));

        let id = self.id.to_string();

        for resp in &resps{
            if let Some(Value::Object(ref m)) = resp.success{
                for (k, v) in m{
                    let mut k_iter = k.split('/');
                    if k_iter.next() == Some("") && k_iter.next() == Some("lights")
                    && k_iter.next() == Some(&*id) && k_iter.next() == Some("state"){
                        if let Some(field) = k_iter.next(){
                            match field{
                                "on"  => self.light.state.on  = v.as_bool().unwrap(),
                                "bri" => self.light.state.bri = v.as_u64().unwrap() as u8,
                                "hue" => self.light.state.hue = v.as_u64().unwrap() as u16,
                                "sat" => self.light.state.sat = v.as_u64().unwrap() as u8,
                                "ct"  => self.light.state.ct  = v.as_u64().map(|v| v as u16),
                                _ => ()
                            }
                        }
                    }
                }
            }
        }
        Ok(resps)
    }
}

#[derive(Debug)]
/// The bridge connection
pub struct Bridge {
    client: Client,
    /// The IP address of the bridge
    pub ip: String,
    /// The username for the user on the bridge
    pub username: String,
}

impl Bridge {
    /// Creates a `Bridge` on the given IP with the given username
    pub fn new<S: Into<String>, U: Into<String>>(ip: S, username: U) -> Self{
        Bridge{
            client: Client::new(),
            ip: ip.into(),
            username: username.into()
        }
    }
    /// Gets all lights that are connected to the bridge
    pub fn get_lights(&self) -> Result<Vec<IdentifiedLight>, HueError> {
        self.client
            .get(&format!("http://{}/api/{}/lights", self.ip, self.username))
            .send()
            .map_err(HueError::from)
            .and_then(|ref mut resp| from_reader::<_, Map<usize, Light>>(resp).map_err(From::from))
            .map(|json: Map<usize, Light>| {
                let mut lights: Vec<_> = json
                    .into_iter()
                    .map(|(id, light)| {
                        IdentifiedLight {
                            id: id,
                            light: light,
                        }
                    })
                    .collect();
                lights.sort_by_key(|x| x.id);
                lights
            })
    }
}
