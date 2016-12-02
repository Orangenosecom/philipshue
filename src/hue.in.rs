use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize)]
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

#[derive(Debug, Deserialize)]
/// A response that either is an error or a success
pub struct HueResponse<T: Serialize + Deserialize>{
    /// The result from the bridge if it didn't fail
    pub success: Option<T>,
    /// The error that was returned from the bridge
    pub error: Option<Error>
}

#[derive(Debug, Deserialize, Serialize)]
/// A user object returned from the API
pub struct User{
    /// The username of the user
    pub username: String
}

#[derive(Debug, Deserialize)]
/// An error object returned from the API
pub struct Error {
    /// The URI the error happened on
    pub address: String,
    /// A short description of the error
    pub description: String,
    /// Its errorcode
    #[serde(rename="type")]
    pub code: u16,
}
