use hyper;
use std::convert::From;
use serde_json;
use std::io;

impl From<::json::Error> for HueError {
    fn from(e: ::json::Error) -> HueError {
        HueErrorKind::BridgeError {
                address: e.address,
                description: e.description,
                error: From::from(e.code),
            }
            .into()
    }
}

error_chain! {
    types {
        HueError, HueErrorKind, ResultExt, Result;
    }

    errors {
        /// An error that occured in the bridge
        #[allow(missing_docs)]
        BridgeError {
            address: String,
            description: String,
            error: BridgeError
        } {
            description("bridge error")
            display("Bridge error {:?} on {}: {}", error, address, description)
        }
    }

    foreign_links {
        JsonError(serde_json::Error) #[doc = "Json error"];
        HyperError(hyper::Error)     #[doc = "Hyper error"];
        IOError(io::Error)           #[doc = "IO error"];
    }
}

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
    pub enum BridgeError {
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
    use self::BridgeError::*;

    assert_eq!(BridgeError::from(101), LinkButtonNotPressed);
    assert_eq!(BridgeError::from(0), Other);
    assert_eq!(BridgeError::from(51234), Other);
    assert_eq!(BridgeError::from(4), MethodNotAvailableForResource);
    assert_eq!(SceneCouldNotBeRemoved as u16, 403);
    assert_eq!(InternalError as u16, 901);
}
