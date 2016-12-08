#[derive(Debug, Clone, Serialize, Deserialize)]
/// The state of the light with similar structure to `LightCommand`
pub struct LightState {
    /// Whether the light is on
    pub on: bool,
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    pub bri: u8,
    /// Hue of the light. Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    pub hue: Option<u16>,
    /// Staturation of the light. 254 is the most saturated (colored) and 0 is the least (white).
    pub sat: Option<u8>,
    /// The x and y coordinates of a colour in [CIE space](http://www.developers.meethue.com/documentation/core-concepts#color_gets_more_complicated)
    pub xy: Option<(f32, f32)>,
    /// The [mired](http://en.wikipedia.org/wiki/Mired) colour temperature of the light.
    pub ct: Option<u16>,
    /// The [alert effect](http://www.developers.meethue.com/documentation/core-concepts#some_extra_fun_stuff)
    pub alert: String,
    /// The dynamic effect of the light. It can be either "none" or "colorloop"
    ///
    /// If "colorloop", the light will cycle hues
    pub effect: Option<String>,
    /// The current colour mode either: "hs" for hue and saturation, "xy" for x and y coordinates in colour space, or "ct" for colour temperature
    pub colormode: Option<String>,
    /// Whether the light can be reached by the bridge
    pub reachable: bool,
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
    pub on: Option<bool>,
    /// Brightness of the colour of the light
    pub bri: Option<u8>,
    /// The hue of the colour of the light
    pub hue: Option<u16>,
    /// The saturation of the colour of the light
    pub sat: Option<u8>,
    /// The x and y coordinates of a colour in [CIE space](http://www.developers.meethue.com/documentation/core-concepts#color_gets_more_complicated)
    pub xy: Option<(f32, f32)>,
    /// The Mired Color temperature of the light. 2012 connected lights are capable of 153 (6500K) to 500 (2000K).
    pub ct: Option<u16>,
    /// The [alert effect](http://www.developers.meethue.com/documentation/core-concepts#some_extra_fun_stuff)
    pub alert: Option<String>,
    /// The dynamic effect of the light. It can be either "none" or "colorloop"
    ///
    /// If "colorloop", the light will cycle hues
    pub effect: Option<String>,
    /// Has to be a value between -254 and 254. Increments or decrements the value of the brightness.
    pub bri_inc: Option<i16>,
    /// Has to be a value between -254 and 254. Increments or decrements the value of the saturation.
    pub sat_inc: Option<i16>,
    /// Has to be a value between -65534 and 65534. Increments or decrements the value of the hue.
    pub hue_inc: Option<i16>,
    /// Has to be a value between -65534 and 65534. Increments or decrements the value of the colour temperature.
    pub ct_inc: Option<i16>,
    /// Increments or decrements the value of the xy.
    pub xy_inc: Option<(i16, i16)>,
    /// The scene identifier to be called (only for used groups)
    pub scene: Option<String>
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
    pub action: Option<LightCommand>,
    /// State reprensentation of the group
    pub state: Option<GroupState>,
    /// The class of the room, if the type of the group is `Room`
    pub class: Option<RoomClass>
}

#[derive(Debug, Clone, Serialize)]
/// Attributes of a group to be changed using `set_group_attributes()`
pub struct GroupCommand {
    /// The new name for the group.
    pub name: Option<String>,
    /// IDs of all the lights that should be in the group.
    pub lights: Vec<usize>,
    /// The class of the room. Default is `Other`.
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
    pub bri: Option<u8>,
    /// Last time the state of at least one light in the group was changed.
    pub lastupdated: Option<String>,
    /// Last time the group was turned on or off.
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

pub use serde_json::{Map, Value as JsonValue};

#[derive(Debug, Clone, Deserialize)]
/// Contains information about what can be updated
pub struct DeviceTypes {
    /// Whether there is an update available for the bridge.
    bridge: bool,
    /// List of lights to be updated.
    lights: Vec<Light>,
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
    pub whitelist: Map<String, WhitelistUser>,
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

#[derive(Debug, Clone, Serialize)]
/// Changes to be applied to the configuration.
///
/// This is parsed to `bridge::modify_configuration()`
pub struct ConfigurationModifier {
    /// Name of the bridge. This is also its uPnP name.
    pub name: Option<String>,
    /// Contains information about software updates
    pub swupdate: Option<SoftwareUpdateModifier>,
    /// IP Address of the proxy server being used or "none".
    pub proxyaddress: Option<String>,
    /// Port of the proxy being used or 0 if no proxy is being used
    pub proxyport: Option<u16>,
    /// Whether the linkbuttion has been preseed within the last 30 seconds.
    pub linkbutton: Option<bool>,
    /// IP address of the bridge.
    pub ipaddress: Option<String>,
    /// Network mask of the bridge.
    pub netmask: Option<String>,
    /// Gateway IP address of the bridge.
    pub gateway: Option<String>,
    /// Whether the IP address of the bridge is obtained via DHCP.
    pub dhcp: Option<bool>,
    /// Current time stored on the bridge.
    ///
    /// **Only modifiable when bridge cannot access the internet.**
    #[serde(rename="UTC")]
    pub utc: Option<String>,
    /// Timezone of the bridge.
    pub timezone: Option<String>,
    /// Perform a touchlink action if true.
    pub touchlink: Option<bool>
}

fn null_value() -> JsonValue{
    JsonValue::Null
}

#[derive(Debug, Clone, Deserialize)]
/// The entire datastore of the bridge.
pub struct FullState {
    /// All lights on the bridge.
    pub lights: Map<usize, Light>,
    /// All groups on the bridge.
    pub groups: Map<usize, Group>,
    /// The configuration of the bridge.
    pub config: Configuration,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub schedule: JsonValue,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub scenes: JsonValue,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub sensors: JsonValue,
    /// Not yet fully implemented
    #[serde(default = "null_value")]
    pub rules: JsonValue
}
