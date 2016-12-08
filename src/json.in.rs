#[derive(Debug, Deserialize)]
/// A user object returned from the API
pub struct User{
    /// The username of the user
    pub username: String
}

#[derive(Debug, Deserialize)]
/// An object containing the ID of a newly created Group
pub struct GroupId{
    /// The ID of the group
    pub id: usize
}

use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A response that either is an error or a success
pub struct HueResponse<T: Deserialize>{
    /// The result from the bridge if it didn't fail
    pub success: Option<T>,
    /// The error that was returned from the bridge
    pub error: Option<Error>
}

use ::errors::HueError;

impl<T: Deserialize> HueResponse<T> {
    pub fn into_result(self) -> Result<T, HueError> {
        if let Some(t) = self.success{
            Ok(t)
        }else if let Some(error) = self.error{
            Err(error.into())
        }else{
            Err(HueError::MalformedResponse)
        }
    }
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
