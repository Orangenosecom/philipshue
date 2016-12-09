use hyper;
use std::convert::From;
use serde_json;
use std::num::ParseIntError;

/// an error returned from the bridge
#[derive(Debug)]
pub struct BridgeError {
    /// The URI the error happened on
    address: String,
    /// A short description of the error
    description: String,
    /// The `BridgeError`
    error: BridgeErrorCode
}

impl From<::json::Error> for HueError {
    fn from(e: ::json::Error) -> HueError {
        HueErrorKind::BridgeError(BridgeError {
            address:e.address,
            description:e.description,
            error:From::from(e.code),
        }).into()
    }
}

error_chain! {
    
    types {
        HueError, HueErrorKind, ResultExt;
    }

    errors {
        /// The response from the bridge was malformed
        ///
        /// This doesn't happen in practice
        MalformedResponse { }
        /// An error that occured in the bridge
        BridgeError(b:BridgeError) {
            description("bridge error")
            display("Bridge error: '{:?}'", b)
        }
    }
    
    foreign_links {
        JsonError(serde_json::Error);
        HyperError(hyper::Error);
        ParseIntError(ParseIntError);
    }
    
}

/// compacted Result type for our HueError
pub type Result<T> = ::std::result::Result<T, HueError>;

macro_rules! error_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident{
            $($err:ident = $n:expr),+;
            $other:ident
        }
    ) => (
        $(#[$meta])*
        pub enum $name{
            $($err = $n,)+
            $other
        }
        impl From<u16> for $name{
            fn from(n: u16) -> Self{
                match n {
                    $($n => $name::$err,)+
                    _ => $name::$other
                }
            }
        }
    );
}

error_enum!{
    /// All errors defined in http://www.developers.meethue.com/documentation/error-messages
    #[repr(u16)]
    #[allow(missing_docs)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BridgeErrorCode {
        // Generic Errors
        UnauthorizedUser = 1,
        BodyContainsInvalidJson = 2,
        ResourceNotAvailable = 3,
        MethodNotAvailableForResource = 4,
        MissingParametersInBody = 5,
        ParameterNotAvailable = 6,
        InvalidValueForParameter = 7,
        ParameterIsNotModifiable = 8,
        TooManyItemsInList = 11,
        ProtalConnectionRequired = 12,
        InternalError = 901,

        // Command Specific Errors
        LinkButtonNotPressed = 101,
        DHCPCannotBeDisabled = 110,
        InvalidUpdateState = 111,
        DeviceIsSetToOff = 201,
        GroupCouldNotBeCreatedGroupFull = 301,
        DeviceCouldNotBeAddedGroupFull = 302,
        DeviceIsUnreachable = 304,
        UpdateOrDeleteGroupOfThisTypeNotAllowed = 305,
        LightAlreadyUsed = 306,
        SceneCouldNotBeCreated = 401,
        SceneCouldNotBeCreatedBufferFull = 402,
        SceneCouldNotBeRemoved = 403,
        NotAllowedToCreateSensorType = 501,
        SensorListIsFull = 502,
        RuleEngineFull = 601,
        ConditionError = 607,
        ActionError = 608,
        UnableToActivae = 609,
        ScheduleListIsFull = 701,
        ScheduleTimezoneNotValid = 702,
        ScheduleCannotSetTimeAndLocalTime = 703,
        CannotCreateSchedule = 704,
        CannotEnableScheduleTimeInPast = 705,
        CommandError = 706,
        SourceModelInvalid = 801,
        SourceFactoryNew = 802,
        InvalidState = 803;
        Other
    }
}

#[test]
fn bridge_errors() {
    use self::BridgeErrorCode::*;

    assert_eq!(BridgeErrorCode::from(101), LinkButtonNotPressed);
    assert_eq!(BridgeErrorCode::from(0), Other);
    assert_eq!(BridgeErrorCode::from(51234), Other);
    assert_eq!(BridgeErrorCode::from(4), MethodNotAvailableForResource);
    assert_eq!(SceneCouldNotBeRemoved as u16, 403);
    assert_eq!(InternalError as u16, 901);
}
