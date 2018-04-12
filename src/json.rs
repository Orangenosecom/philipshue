#[derive(Debug, Deserialize)]
/// A user object returned from the API
pub struct User{
    /// The username of the user
    pub username: String
}

#[derive(Debug, Deserialize)]
/// An object containing the ID of something newly created
pub struct Id<T>
{
    /// The ID
    pub id: T
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
/// A response that either is an error or a success
pub enum HueResponse<T> {
    /// The result from the bridge if it didn't fail
    Success(T),
    /// The error that was returned from the bridge
    Error(Error)
}

use ::errors::HueError;

impl<T> HueResponse<T> {
    pub fn into_result(self) -> Result<T, HueError> {
        match self {
            HueResponse::Success(s) => Ok(s),
            HueResponse::Error(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SceneRecall<'a> {
    pub scene: &'a str
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
