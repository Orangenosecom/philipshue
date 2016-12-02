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
/// Discover bridge IP using UPnP
///
/// Waits for about 5 seconds to make sure it gets a response
#[cfg(feature = "ssdp")]
pub fn discover_upnp() -> Result<Vec<String>, ::ssdp::SSDPError>{
    use ssdp::header::{HeaderMut, Man, MX, ST};
    use ssdp::message::SearchRequest;
    use ssdp::FieldMap;

    let mut request = SearchRequest::new();

    request.set(Man);
    request.set(MX(5));
    request.set(ST::Target(FieldMap::upnp("IpBridge")));

    request.multicast().map(|r| r
        .into_iter()
        .map(|(_, src)| src.ip().to_string())
        .collect())
}

/// Tries to register a user, returning the username if successful
///
/// This usually returns a `HueError::BridgeError` saying the link button needs to be pressed.
/// Therefore it recommended to call this function in a loop:
/// ## Example
/// ```no_run
/// use philipshue::errors::{HueError, BridgeError};
/// use philipshue::bridge::{self, Bridge};
///
/// let mut bridge = None;
/// // Discover a bridge
/// let bridge_ip = philipshue::bridge::discover().unwrap().pop().unwrap().into_ip();
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

#[derive(Debug)]
/// The bridge connection
pub struct Bridge {
    client: Client,
    url: String
}

fn send_with_body<'a, T: Deserialize>(rb: RequestBuilder<'a>, body: &'a str) -> Result<T, HueError> {
    send(rb.body(Body::BufBody(body.as_bytes(), body.len())))
}

fn send<T: Deserialize>(rb: RequestBuilder) -> Result<T, HueError> {
    rb.send()
      .map_err(HueError::from)
      .and_then(|ref mut resp| from_reader::<_, T>(resp).map_err(From::from))
}

#[test]
fn get_ip_and_username() {
    let b = Bridge::new("test", "hello");
    assert_eq!(b.get_ip(), "test");
    assert_eq!(b.get_username(), "hello");
}

use serde::Deserialize;
use hyper::client::RequestBuilder;
use ::clean::clean_json;

impl Bridge {
    /// Creates a `Bridge` on the given IP with the given username
    pub fn new<S: Into<String>, U: Into<String>>(ip: S, username: U) -> Self{
        Bridge{
            client: Client::new(),
            url: format!("http://{}/api/{}/", ip.into(), username.into())
        }
    }
    /// Gets the IP of bridge
    pub fn get_ip(&self) -> &str{
        self.url.split('/').nth(2).unwrap()
    }
    /// Gets the username this `Bridge` uses
    pub fn get_username(&self) -> &str{
        self.url.split('/').nth(4).unwrap()
    }
    /// Gets all lights that are connected to the bridge
    pub fn get_all_lights(&self) -> Result<Map<usize, Light>, HueError> {
        send(self.client.get(&format!("{}lights", self.url)))
    }
    /// Gets the light with the specific id
    pub fn get_light(&self, id: usize) -> Result<Light, HueError> {
        send(self.client.get(&format!("{}lights/{}", self.url, id)))
    }
    /// Gets all the light that were found last time a search for new lights was done
    pub fn get_new_lights(&self) -> Result<Map<usize, Light>, HueError> {
        // TODO return lastscan too
        send(self.client.get(&format!("{}lights/new", self.url)))
    }
    /// Makes the bridge search for new lights (and switches).
    ///
    /// The found lights can be retrieved with `get_new_lights()`
    pub fn search_for_new_lights(&self) -> Result<Vec<HueResponse<Value>>, HueError> {
        // TODO Allow deviceids to be specified
        send(self.client.post(&format!("{}lights", self.url)))
    }
    /// Sets the state of a light by sending a `LightCommand` to the bridge for this light
    pub fn set_light_state(&self, id: usize, command: LightCommand) -> Result<Vec<HueResponse<Value>>, HueError>{
        send_with_body(self.client.put(&format!("{}lights/{}/state", self.url, id)), &clean_json(to_string(&command)?))
    }
    /// Renames the light
    pub fn rename_light(&self, id: usize, name: String) -> Result<Vec<HueResponse<Value>>, HueError> {
        let mut name_map = Map::new();
        name_map.insert("name".to_owned(), name);
        send_with_body(self.client.put(&format!("{}lights/{}", self.url, id)), &clean_json(to_string(&name_map)?))
    }
    /// Deletes a light from the bridge
    pub fn delete_light(&self, id: usize) -> Result<Vec<HueResponse<Value>>, HueError> {
        send(self.client.delete(&format!("{}lights/{}", self.url, id)))
    }
}
