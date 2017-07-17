use serde::de::{Deserialize, Deserializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The state of the light with similar structure to `LightCommand`
pub struct LightState {
    /// Whether the light is on
    pub on: bool,
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    pub bri: u8,
    /// Hue of the light. Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    /// Staturation of the light. 254 is the most saturated (colored) and 0 is the least (white).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    /// The x and y coordinates of a colour in [CIE space](http://www.developers.meethue.com/documentation/core-concepts#color_gets_more_complicated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<(f32, f32)>,
    /// The [mired](http://en.wikipedia.org/wiki/Mired) colour temperature of the light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
    /// The [alert effect](http://www.developers.meethue.com/documentation/core-concepts#some_extra_fun_stuff)
    pub alert: String,
    /// The dynamic effect of the light. It can be either "none" or "colorloop"
    ///
    /// If "colorloop", the light will cycle hues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// The current colour mode either: "hs" for hue and saturation, "xy" for x and y coordinates in colour space, or "ct" for colour temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colormode: Option<String>,
    /// Whether the light can be reached by the bridge
    pub reachable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The state of the light. Same as `LightState` except there's no `reachable` field.
pub struct LightStateChange {
    /// Whether the light is on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    /// Hue of the light. Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    /// Staturation of the light. 254 is the most saturated (colored) and 0 is the least (white).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    /// The x and y coordinates of a colour in [CIE space](http://www.developers.meethue.com/documentation/core-concepts#color_gets_more_complicated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<(f32, f32)>,
    /// The [mired](http://en.wikipedia.org/wiki/Mired) colour temperature of the light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
    /// The [alert effect](http://www.developers.meethue.com/documentation/core-concepts#some_extra_fun_stuff)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    /// The dynamic effect of the light. It can be either "none" or "colorloop"
    ///
    /// If "colorloop", the light will cycle hues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// The current colour mode either: "hs" for hue and saturation, "xy" for x and y coordinates in colour space, or "ct" for colour temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colormode: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Details about a specific light
pub struct Light {
    /// The unique name given to the light
    pub name: String,
    /// The hardware model of the light
    pub modelid: String,
    /// The version of the software running on the light
    pub swversion: String,
    /// Unique ID of the device
    pub uniqueid: String,
    /// The state of the light (See `LightState` for more)
    pub state: LightState
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Struct for building a command that will be sent to the Hue bridge telling it what to do with a light
///
/// View [the lights-api documention](http://www.developers.meethue.com/documentation/lights-api) for more information
pub struct LightCommand {
    /// Whether to turn the light off or on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    /// Brightness of the colour of the light
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    /// The hue of the colour of the light
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    /// The saturation of the colour of the light
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    /// The x and y coordinates of a colour in [CIE space](http://www.developers.meethue.com/documentation/core-concepts#color_gets_more_complicated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<(f32, f32)>,
    /// The Mired Color temperature of the light. 2012 connected lights are capable of 153 (6500K) to 500 (2000K).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
    /// The [alert effect](http://www.developers.meethue.com/documentation/core-concepts#some_extra_fun_stuff)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    /// The dynamic effect of the light. It can be either "none" or "colorloop"
    ///
    /// If "colorloop", the light will cycle hues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Has to be a value between -254 and 254. Increments or decrements the value of the brightness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri_inc: Option<i16>,
    /// Has to be a value between -254 and 254. Increments or decrements the value of the saturation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_inc: Option<i16>,
    /// Has to be a value between -65534 and 65534. Increments or decrements the value of the hue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue_inc: Option<i16>,
    /// Has to be a value between -65534 and 65534. Increments or decrements the value of the colour temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_inc: Option<i16>,
    /// Increments or decrements the value of the xy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy_inc: Option<(i16, i16)>,
}

impl LightCommand {
    /// Returns a `LightCommand` that turns a light on
    pub fn on(self) -> Self {
        LightCommand { on: Some(true), ..self }
    }
    /// Returns a `LightCommand` that turns a light on
    pub fn off(self) -> Self {
        LightCommand { on: Some(false), ..self }
    }
    /// Sets the brightness to set the light to
    pub fn with_bri(self, b: u8) -> Self {
        LightCommand { bri: Some(b), ..self }
    }
    /// Sets the hue to set the light to
    pub fn with_hue(self, h: u16) -> Self {
        LightCommand { hue: Some(h), ..self }
    }
    /// Sets the saturation to set the light to
    pub fn with_sat(self, s: u8) -> Self {
        LightCommand { sat: Some(s), ..self }
    }
    /// Sets the xy colour coordinates to set the light to
    pub fn with_xy(self, xy: (f32, f32)) -> Self {
        LightCommand { xy: Some(xy), ..self }
    }
    /// Sets the temperature to set the light to
    pub fn with_ct(self, c: u16) -> Self {
        LightCommand { ct: Some(c), ..self }
    }
    /// Sets the alert mode to set the light to
    pub fn with_alert(self, a: String) -> Self {
        LightCommand { alert: Some(a), ..self }
    }
    /// Sets the effect mode to set the light to
    pub fn with_effect(self, a: String) -> Self {
        LightCommand { effect: Some(a), ..self }
    }
    /// Sets the brightness increment value
    pub fn with_bri_inc(self, b: i16) -> Self {
        LightCommand { bri_inc: Some(b), ..self }
    }
    /// Sets the hue increment value
    pub fn with_hue_inc(self, h: i16) -> Self {
        LightCommand { hue_inc: Some(h), ..self }
    }
    /// Sets the saturation increment value
    pub fn with_sat_inc(self, s: i16) -> Self {
        LightCommand { sat_inc: Some(s), ..self }
    }
    /// Sets the saturation increment value
    pub fn with_ct_inc(self, ct: i16) -> Self {
        LightCommand { ct_inc: Some(ct), ..self }
    }
    /// Sets the x and y increment value
    pub fn with_xy_inc(self, xy: (i16, i16)) -> Self {
        LightCommand { xy_inc: Some(xy), ..self }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Type of a group
pub enum GroupType{
    /// Multisource luminaire group.
    Luminaire,
    /// A sub group of multisource luminaire lights.
    LightSource,
    /// A simple group of lights that can be controlled together.
    LightGroup,
    /// A group of lights that are physically in the same room.
    Room
}

use std::fmt::{self, Display};

impl Display for GroupType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GroupType::*;
        match *self{
            Luminaire => "Luminaire",
            LightSource => "LightSource",
            LightGroup => "LightGroup",
            Room => "Room"
        }.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(missing_docs)]
/// Class of the room of a group supported by the Hue API
pub enum RoomClass{
    #[serde(rename = "Living room")]
    LivingRoom,
    Kitchen,
    Dining,
    Bedroom,
    #[serde(rename = "Kids bedroom")]
    KidsBedroom,
    Bathroom,
    Nursery,
    Recreation,
    Office,
    Gym,
    Hallway,
    Toilet,
    #[serde(rename = "Front door")]
    FrontDoor,
    Garage,
    Terrace,
    Garden,
    Driveway,
    Carport,
    Other
}

impl Display for RoomClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RoomClass::*;
        match *self{
            LivingRoom => "Living room",
            Kitchen => "Kitchen",
            Dining => "Dining",
            Bedroom => "Bedroom",
            KidsBedroom => "Kids bedroom",
            Bathroom => "Bathroom",
            Nursery => "Nursery",
            Recreation => "Recreation",
            Office => "Office",
            Gym => "Gym",
            Hallway => "Hallway",
            Toilet => "Toilet",
            FrontDoor => "Front door",
            Garage => "Garage",
            Terrace => "Terrace",
            Garden => "Garden",
            Driveway => "Driveway",
            Carport => "Carport",
            Other => "Other"
        }.fmt(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A reprensentation of a Hue group of lights
pub struct Group {
    /// Name of the group. (Default name is "Group").
    pub name: String,
    /// IDs of all the lights in this group
    pub lights: Vec<usize>,
    #[serde(rename="type")]
    /// Type of the group
    pub group_type: GroupType,
    // Actually just a `LightState` without the `reachable` field
    /// The `LightCommand` applied to all lights in the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LightStateChange>,
    /// State reprensentation of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GroupState>,
    /// The class of the room, if the type of the group is `Room`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<RoomClass>
}

#[derive(Debug, Clone, Serialize)]
/// Attributes of a group to be changed using `set_group_attributes()`
pub struct GroupCommand {
    /// The new name for the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// IDs of all the lights that should be in the group.
    pub lights: Vec<usize>,
    /// The class of the room. Default is `Other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<RoomClass>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// State reprensentation of the group
pub struct GroupState {
    /// `false` if all lamps are off, `true` otherwise.
    pub any_on: bool,
    /// `true` only if all lamps are on.
    pub all_on: bool,
    /// The average brightness of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    /// Last time the state of at least one light in the group was changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastupdated: Option<String>,
    /// Last time the group was turned on or off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastswitched: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Responses from the `discover` function
pub struct Discovery{
    /// The ID of the bridge
    pub id: String,
    /// The local IP address of the bridge
    pub internalipaddress: String
}

impl Discovery {
    /// The ip of this discovered bridge
    pub fn ip(&self) -> &str{
        &self.internalipaddress
    }
    /// The id of this discovered bridge
    pub fn id(&self) -> &str{
        &self.id
    }
    /// Consumes self and returns the IP
    pub fn into_ip(self) -> String{
        let Discovery{internalipaddress, ..} = self;
        internalipaddress
    }
}

pub use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
/// Contains information about what can be updated
pub struct DeviceTypes {
    /// Whether there is an update available for the bridge.
    bridge: bool,
    /// List of lights to be updated.
    lights: Vec<usize>,
}

#[derive(Debug, Clone, Deserialize)]
/// Information about software updates on the bridge
pub struct SoftwareUpdate {
    /// Lets the bridge search for software updates
    pub checkforupdate: bool,
    /// Details about the types of updates available
    pub devicetypes: DeviceTypes,
    // FIXME What are those?
    /// ?
    pub updatestate: u8,
    /// ?
    pub url: String,
    /// ?
    pub text: String,
    /// ?
    pub notify: bool,
}

#[derive(Debug, Clone, Deserialize)]
/// A user in the whitelist of a `Configuration`
pub struct WhitelistUser {
    /// Name of the user. It's what you specify as `devicetype` when registering a user
    pub name: String,
    /// Date this user was last used
    #[serde(rename="last use date")]
    pub last_use_date: String,
    /// Date this user was created
    #[serde(rename="create date")]
    pub create_date: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Configuration of the bridge
pub struct Configuration {
    /// Name of the bridge. This is also its uPnP name.
    pub name: String,
    /// Contains information about software updates
    pub swupdate: SoftwareUpdate,
    /// A list of all registered users
    pub whitelist: BTreeMap<String, WhitelistUser>,
    /// Version of the hue API on the bridge.
    pub apiversion: String,
    /// Software version of the bridge
    pub swversion: String,
    /// IP Address of the proxy server being used or "none".
    pub proxyaddress: String,
    /// Port of the proxy being used or 0 if no proxy is being used
    pub proxyport: u16,
    /// Whether the linkbuttion has been preseed within the last 30 seconds.
    pub linkbutton: bool,
    /// IP address of the bridge.
    pub ipaddress: String,
    /// MAC address of the bridge.
    pub mac: String,
    /// Network mask of the bridge.
    pub netmask: String,
    /// Gateway IP address of the bridge.
    pub gateway: String,
    /// Whether the IP address of the bridge is obtained via DHCP.
    pub dhcp: bool,
    /// Whether the bridge is registered to synchronize data with a portal account.
    pub portalservices: bool,
    /// Current time stored on the bridge.
    #[serde(rename="UTC")]
    pub utc: String,
    /// The local time of the bridge or "none".
    pub localtime: String,
    /// Timezone of the bridge as OlsenIDs (e.g. "Europe/Amsterdam") or "none".
    pub timezone: String,
    /// The current wireless frequency channel used by the bridge. It can take values of 11, 15, 20,25 or 0 if undefined (factory new).
    pub zigbeechannel: u8,
    /// This parameter uniquely identifies the hardware model of the bridge (BSB001, BSB002).
    pub modelid: String,
    /// The unique bridge id. This is currently generated from the bridge Ethernet MAC address.
    pub bridgeid: String,
    /// Whether bridge settings are factory new.
    pub factorynew: bool,
    /// If a bridge backup file has been restored on this bridge from a bridge with a different bridgeid, it will indicate that bridge id.
    pub replacesbridgeid: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
/// Information to set about software updates on the bridge
pub struct SoftwareUpdateModifier {
    /// Lets the bridge search for software updates
    pub checkforupdate: bool
}

#[derive(Debug, Clone, Serialize, Default)]
/// Changes to be applied to the configuration.
///
/// This is parsed to `bridge::modify_configuration()`
pub struct ConfigurationModifier {
    /// Name of the bridge. This is also its uPnP name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contains information about software updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swupdate: Option<SoftwareUpdateModifier>,
    /// IP Address of the proxy server being used or "none".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxyaddress: Option<String>,
    /// Port of the proxy being used or 0 if no proxy is being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxyport: Option<u16>,
    /// Whether the linkbuttion has been preseed within the last 30 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkbutton: Option<bool>,
    /// IP address of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipaddress: Option<String>,
    /// Network mask of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    /// Gateway IP address of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Whether the IP address of the bridge is obtained via DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    /// Current time stored on the bridge.
    ///
    /// **Only modifiable when bridge cannot access the internet.**
    #[serde(rename="UTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc: Option<String>,
    /// Timezone of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Perform a touchlink action if true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touchlink: Option<bool>
}

fn null_value() -> JsonValue{
    JsonValue::Null
}

#[derive(Debug, Clone, Deserialize)]
/// The entire datastore of the bridge.
pub struct FullState {
    /// All lights on the bridge.
    pub lights: BTreeMap<usize, Light>,
    /// All groups on the bridge.
    pub groups: BTreeMap<usize, Group>,
    /// The configuration of the bridge.
    pub config: Configuration,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub schedule: JsonValue,
    /// All scenes on the bridge
    pub scenes: BTreeMap<String, Scene>,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub sensors: JsonValue,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub rules: JsonValue
}

/// A [scene](https://developers.meethue.com/documentation/scenes-api)
///
/// A scene can be used to store a specific set of states of lights on the bridge to recall later.
#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    /// Human readable name given to the scene
    pub name: String,
    /// The IDs of the lights in the scene.
    pub lights: Vec<usize>,
    /// The name of the user that created or last modified the scene
    pub owner: String,
    /// Whether the scene can be deleted automatically by the bridge
    pub recycle: bool,
    /// Whether the scene is locked by a rule or schedule.
    pub locked: bool,
    /// App specific data linked to this scene
    #[serde(deserialize_with = "non_default")]
    pub appdata: Option<AppData>,
    /// Reserved for future use. See Philips Hue documention
    pub picture: Option<String>,
    /// UTC timestamp of when the scene was last updated
    pub lastupdated: Option<String>,
    /// Light states stored on the scene to be recalled
    #[serde(default)]
    pub lightstates: BTreeMap<usize, LightStateChange>
}

fn non_default<'a, 'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where T: Deserialize<'de> + PartialEq + Default, D: Deserializer<'de> {
    let ad = <Option<T>>::deserialize(de)?;
    if ad.as_ref().map(|x| *x == Default::default()).unwrap_or(true) {
        Ok(None)
    } else {
        Ok(ad)
    }
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
/// App specific data linked to a scene
pub struct AppData {
    /// App specific version of the data field.
    #[serde(default)]
    pub version: i8,
    /// App specific data. Can be anything.
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Serialize)]
/// A [scene](https://developers.meethue.com/documentation/scenes-api)
pub struct SceneCreater {
    /// Human readable name.
    pub name: String,
    /// IDs of the lights the scene uses.
    pub lights: Vec<usize>,
    /// Whether the bridge can just delete this scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle: Option<bool>,
    /// Application specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appdata: Option<AppData>,
    /// Picture for the scene
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    /// Duration of time (in deciseconds) for the lights to transition from one state to another with this scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitiontime: Option<u16>
}
#[derive(Debug, Clone, Serialize)]
/// Struct for modifying a scene (renaming, setting lights, updating their state).
pub struct SceneModifier {
    /// Name to rename the scene to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New IDs of the lights the scene uses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<usize>>,
    /// If true, it will update the light states in the scene to the states of the actual lights
    #[serde(skip_serializing_if = "::std::ops::Not::not")]
    pub storelightstate: bool
}
