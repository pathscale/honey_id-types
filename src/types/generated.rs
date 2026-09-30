use endpoint_libs::libs::error_code::ErrorCode;
use endpoint_libs::libs::types::*;
use endpoint_libs::libs::ws::toolbox::CustomError;
use endpoint_libs::libs::ws::*;
use num_derive::FromPrimitive;
use serde::*;
use strum_macros::{Display, EnumString};

use psc_nanoid::{alphabet::Base62Alphabet, Nanoid};
use rkyv::Archive;
use std::net::IpAddr;
use worktable::prelude::*;

#[derive(
    MemStat,
    Archive,
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    Ord,
    EnumString,
    rkyv::Deserialize,
    rkyv::Serialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[repr(u8)]
pub enum LogLevel {
    /// Logging disabled.
    Off = 0,
    /// Error level logging.
    Error = 1,
    /// Warning level logging.
    Warn = 2,
    /// Info level logging.
    Info = 3,
    /// Debug level logging.
    Debug = 4,
    /// Trace level logging.
    Trace = 5,
    /// Detailed trace logging (no crate filtering).
    Detail = 6,
}

#[derive(
    MemStat,
    Archive,
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    Ord,
    EnumString,
    rkyv::Deserialize,
    rkyv::Serialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[repr(u8)]
pub enum UserRole {
    /// Public can only view some data.
    Public = 0,
    /// Platform admin can do literally everything. Very dangerous role.
    PlatformAdmin = 1,
    /// Platform support can view and manage some staff.
    PlatformSupport = 2,
    /// New user in application, can only create new app or be invited to an app.
    AppNewUser = 3,
    /// App admin can manage the application, but not the platform.
    AppAdmin = 4,
    /// App support see the application info, but not the platform.
    AppSupport = 5,
    /// The role is used for external users only.
    AppApiKey = 6,
    /// The role is used for platform only.
    Platform = 7,
}

#[derive(
    MemStat,
    Archive,
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    Ord,
    EnumString,
    rkyv::Deserialize,
    rkyv::Serialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[repr(u8)]
pub enum UserStatus {
    /// Active user.
    Enabled = 1,
    /// Inactive user.
    Disabled = 2,
    /// Banned user.
    Banned = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppKeyInfo {
    pub keyId: String,
    pub label: String,
    pub createdAt: i64,
    #[serde(default)]
    pub lastUsedAt: Option<i64>,
    #[serde(default)]
    pub expiresAt: Option<i64>,
    #[serde(default)]
    pub revokedAt: Option<i64>,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, Hash,
)]
pub enum EnumEndpoint {
    PublicConnect = 0,
    Signup = 10,
    SubmitUsername = 12,
    SubmitPassword = 13,
    PlatformConnect = 100,
    CreateAppConfig = 111,
    BanUser = 112,
    UnbanUser = 113,
    DeleteUser = 114,
    DeleteAppConfig = 115,
    EditAppConfig = 116,
    GetAppSecurityRules = 117,
    SetLogLevel = 118,
    RegenerateAppApiKey = 119,
    GetUserSecurity = 120,
    CreateAppKey = 121,
    ListAppKeys = 122,
    RevokeAppKey = 123,
    RotateAppCallbackCredential = 124,
    InspectAppCredential = 125,
    SetAppService = 126,
    ApiKeyConnect = 200,
    AuthorizedConnect = 201,
    ReceiveToken = 210,
    ReceiveUserInfo = 211,
    ReceiveUserDeleted = 212,
    ValidateToken = 213,
}

impl EnumEndpoint {
    pub fn schema(&self) -> endpoint_libs::model::EndpointSchema {
        let schema = match self {
            Self::PublicConnect => PublicConnectRequest::SCHEMA,
            Self::Signup => SignupRequest::SCHEMA,
            Self::SubmitUsername => SubmitUsernameRequest::SCHEMA,
            Self::SubmitPassword => SubmitPasswordRequest::SCHEMA,
            Self::PlatformConnect => PlatformConnectRequest::SCHEMA,
            Self::CreateAppConfig => CreateAppConfigRequest::SCHEMA,
            Self::BanUser => BanUserRequest::SCHEMA,
            Self::UnbanUser => UnbanUserRequest::SCHEMA,
            Self::DeleteUser => DeleteUserRequest::SCHEMA,
            Self::DeleteAppConfig => DeleteAppConfigRequest::SCHEMA,
            Self::EditAppConfig => EditAppConfigRequest::SCHEMA,
            Self::GetAppSecurityRules => GetAppSecurityRulesRequest::SCHEMA,
            Self::SetLogLevel => SetLogLevelRequest::SCHEMA,
            Self::RegenerateAppApiKey => RegenerateAppApiKeyRequest::SCHEMA,
            Self::GetUserSecurity => GetUserSecurityRequest::SCHEMA,
            Self::CreateAppKey => CreateAppKeyRequest::SCHEMA,
            Self::ListAppKeys => ListAppKeysRequest::SCHEMA,
            Self::RevokeAppKey => RevokeAppKeyRequest::SCHEMA,
            Self::RotateAppCallbackCredential => RotateAppCallbackCredentialRequest::SCHEMA,
            Self::InspectAppCredential => InspectAppCredentialRequest::SCHEMA,
            Self::SetAppService => SetAppServiceRequest::SCHEMA,
            Self::ApiKeyConnect => ApiKeyConnectRequest::SCHEMA,
            Self::AuthorizedConnect => AuthorizedConnectRequest::SCHEMA,
            Self::ReceiveToken => ReceiveTokenRequest::SCHEMA,
            Self::ReceiveUserInfo => ReceiveUserInfoRequest::SCHEMA,
            Self::ReceiveUserDeleted => ReceiveUserDeletedRequest::SCHEMA,
            Self::ValidateToken => ValidateTokenRequest::SCHEMA,
        };
        serde_json::from_str(schema).unwrap()
    }
}

/// JSON-serialized shared struct/enum definitions referenced by endpoint schemas.
pub const TYPE_DEFINITIONS: &'static str = r#"[
  {
    "Struct": {
      "name": "AppKeyInfo",
      "fields": [
        {
          "name": "keyId",
          "ty": "String"
        },
        {
          "name": "label",
          "ty": "String"
        },
        {
          "name": "createdAt",
          "ty": "Int64"
        },
        {
          "name": "lastUsedAt",
          "ty": {
            "Optional": "Int64"
          }
        },
        {
          "name": "expiresAt",
          "ty": {
            "Optional": "Int64"
          }
        },
        {
          "name": "revokedAt",
          "ty": {
            "Optional": "Int64"
          }
        }
      ]
    }
  },
  {
    "Enum": {
      "name": "LogLevel",
      "variants": [
        {
          "name": "off",
          "description": "Logging disabled.",
          "value": 0
        },
        {
          "name": "error",
          "description": "Error level logging.",
          "value": 1
        },
        {
          "name": "warn",
          "description": "Warning level logging.",
          "value": 2
        },
        {
          "name": "info",
          "description": "Info level logging.",
          "value": 3
        },
        {
          "name": "debug",
          "description": "Debug level logging.",
          "value": 4
        },
        {
          "name": "trace",
          "description": "Trace level logging.",
          "value": 5
        },
        {
          "name": "detail",
          "description": "Detailed trace logging (no crate filtering).",
          "value": 6
        }
      ]
    }
  },
  {
    "Enum": {
      "name": "UserRole",
      "variants": [
        {
          "name": "Public",
          "description": "Public can only view some data.",
          "value": 0
        },
        {
          "name": "PlatformAdmin",
          "description": "Platform admin can do literally everything. Very dangerous role.",
          "value": 1
        },
        {
          "name": "PlatformSupport",
          "description": "Platform support can view and manage some staff.",
          "value": 2
        },
        {
          "name": "AppNewUser",
          "description": "New user in application, can only create new app or be invited to an app.",
          "value": 3
        },
        {
          "name": "AppAdmin",
          "description": "App admin can manage the application, but not the platform.",
          "value": 4
        },
        {
          "name": "AppSupport",
          "description": "App support see the application info, but not the platform.",
          "value": 5
        },
        {
          "name": "AppApiKey",
          "description": "The role is used for external users only.",
          "value": 6
        },
        {
          "name": "Platform",
          "description": "The role is used for platform only.",
          "value": 7
        }
      ]
    }
  },
  {
    "Enum": {
      "name": "UserStatus",
      "variants": [
        {
          "name": "enabled",
          "description": "Active user.",
          "value": 1
        },
        {
          "name": "disabled",
          "description": "Inactive user.",
          "value": 2
        },
        {
          "name": "banned",
          "description": "Banned user.",
          "value": 3
        }
      ]
    }
  },
  {
    "Enum": {
      "name": "ErrorCode",
      "variants": [
        {
          "name": "BadRequest",
          "description": "Bad request",
          "value": 100400
        },
        {
          "name": "Unauthorized",
          "description": "Authentication is required",
          "value": 100401
        },
        {
          "name": "PaymentRequired",
          "description": "Payment is required",
          "value": 100402
        },
        {
          "name": "Forbidden",
          "description": "Access is forbidden",
          "value": 100403
        },
        {
          "name": "NotFound",
          "description": "Resource was not found",
          "value": 100404
        },
        {
          "name": "MethodNotAllowed",
          "description": "Method is not allowed",
          "value": 100405
        },
        {
          "name": "NotAcceptable",
          "description": "Response format is not acceptable",
          "value": 100406
        },
        {
          "name": "ProxyAuthenticationRequired",
          "description": "Proxy authentication is required",
          "value": 100407
        },
        {
          "name": "RequestTimeout",
          "description": "Request timed out",
          "value": 100408
        },
        {
          "name": "Conflict",
          "description": "Request conflicts with current state",
          "value": 100409
        },
        {
          "name": "Gone",
          "description": "Resource is gone",
          "value": 100410
        },
        {
          "name": "LengthRequired",
          "description": "Content length is required",
          "value": 100411
        },
        {
          "name": "PreconditionFailed",
          "description": "Precondition failed",
          "value": 100412
        },
        {
          "name": "PayloadTooLarge",
          "description": "Payload is too large",
          "value": 100413
        },
        {
          "name": "UriTooLong",
          "description": "URI is too long",
          "value": 100414
        },
        {
          "name": "UnsupportedMediaType",
          "description": "Media type is unsupported",
          "value": 100415
        },
        {
          "name": "RangeNotSatisfiable",
          "description": "Requested range cannot be satisfied",
          "value": 100416
        },
        {
          "name": "ExpectationFailed",
          "description": "Expectation failed",
          "value": 100417
        },
        {
          "name": "ImATeapot",
          "description": "I'm a teapot",
          "value": 100418
        },
        {
          "name": "MisdirectedRequest",
          "description": "Request was misdirected",
          "value": 100421
        },
        {
          "name": "UnprocessableEntity",
          "description": "Entity could not be processed",
          "value": 100422
        },
        {
          "name": "Locked",
          "description": "Resource is locked",
          "value": 100423
        },
        {
          "name": "FailedDependency",
          "description": "Dependency failed",
          "value": 100424
        },
        {
          "name": "UpgradeRequired",
          "description": "Request must be upgraded",
          "value": 100426
        },
        {
          "name": "PreconditionRequired",
          "description": "Precondition is required",
          "value": 100428
        },
        {
          "name": "TooManyRequests",
          "description": "Too many requests",
          "value": 100429
        },
        {
          "name": "RequestHeaderFieldsTooLarge",
          "description": "Request header fields are too large",
          "value": 100431
        },
        {
          "name": "UnavailableForLegalReasons",
          "description": "Unavailable for legal reasons",
          "value": 100451
        },
        {
          "name": "InternalError",
          "description": "Internal server error",
          "value": 100500
        },
        {
          "name": "NotImplemented",
          "description": "Endpoint is not implemented",
          "value": 100501
        },
        {
          "name": "BadGateway",
          "description": "Bad gateway",
          "value": 100502
        },
        {
          "name": "ServiceUnavailable",
          "description": "Service is unavailable",
          "value": 100503
        },
        {
          "name": "GatewayTimeout",
          "description": "Gateway timed out",
          "value": 100504
        },
        {
          "name": "HttpVersionNotSupported",
          "description": "HTTP version is not supported",
          "value": 100505
        },
        {
          "name": "VariantAlsoNegotiates",
          "description": "Content negotiation variant problem",
          "value": 100506
        },
        {
          "name": "InsufficientStorage",
          "description": "Insufficient storage",
          "value": 100507
        },
        {
          "name": "LoopDetected",
          "description": "Loop was detected",
          "value": 100508
        },
        {
          "name": "NotExtended",
          "description": "Request must be extended",
          "value": 100510
        },
        {
          "name": "NetworkAuthenticationRequired",
          "description": "Network authentication is required",
          "value": 100511
        }
      ]
    }
  }
]"#;

/// Builds the type registry over all shared definitions, for use with
/// `WebsocketServer::enable_mcp()`.
pub fn type_registry() -> endpoint_libs::model::TypeRegistry {
    let types: Vec<endpoint_libs::model::Type> =
        serde_json::from_str(TYPE_DEFINITIONS).expect("Invalid embedded type definitions");
    let mut registry = endpoint_libs::model::TypeRegistry::new();
    registry.add_all(types.iter());
    registry
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, Hash,
)]
pub enum EnumErrorCode {
    /// Bad request
    BadRequest = 100400,
    /// Authentication is required
    Unauthorized = 100401,
    /// Payment is required
    PaymentRequired = 100402,
    /// Access is forbidden
    Forbidden = 100403,
    /// Resource was not found
    NotFound = 100404,
    /// Method is not allowed
    MethodNotAllowed = 100405,
    /// Response format is not acceptable
    NotAcceptable = 100406,
    /// Proxy authentication is required
    ProxyAuthenticationRequired = 100407,
    /// Request timed out
    RequestTimeout = 100408,
    /// Request conflicts with current state
    Conflict = 100409,
    /// Resource is gone
    Gone = 100410,
    /// Content length is required
    LengthRequired = 100411,
    /// Precondition failed
    PreconditionFailed = 100412,
    /// Payload is too large
    PayloadTooLarge = 100413,
    /// URI is too long
    UriTooLong = 100414,
    /// Media type is unsupported
    UnsupportedMediaType = 100415,
    /// Requested range cannot be satisfied
    RangeNotSatisfiable = 100416,
    /// Expectation failed
    ExpectationFailed = 100417,
    /// I'm a teapot
    ImATeapot = 100418,
    /// Request was misdirected
    MisdirectedRequest = 100421,
    /// Entity could not be processed
    UnprocessableEntity = 100422,
    /// Resource is locked
    Locked = 100423,
    /// Dependency failed
    FailedDependency = 100424,
    /// Request must be upgraded
    UpgradeRequired = 100426,
    /// Precondition is required
    PreconditionRequired = 100428,
    /// Too many requests
    TooManyRequests = 100429,
    /// Request header fields are too large
    RequestHeaderFieldsTooLarge = 100431,
    /// Unavailable for legal reasons
    UnavailableForLegalReasons = 100451,
    /// Internal server error
    InternalError = 100500,
    /// Endpoint is not implemented
    NotImplemented = 100501,
    /// Bad gateway
    BadGateway = 100502,
    /// Service is unavailable
    ServiceUnavailable = 100503,
    /// Gateway timed out
    GatewayTimeout = 100504,
    /// HTTP version is not supported
    HttpVersionNotSupported = 100505,
    /// Content negotiation variant problem
    VariantAlsoNegotiates = 100506,
    /// Insufficient storage
    InsufficientStorage = 100507,
    /// Loop was detected
    LoopDetected = 100508,
    /// Request must be extended
    NotExtended = 100510,
    /// Network authentication is required
    NetworkAuthenticationRequired = 100511,
}

impl From<EnumErrorCode> for ErrorCode {
    fn from(e: EnumErrorCode) -> Self {
        ErrorCode::new(e as _)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyConnectRequest {
    pub appApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyConnectResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedConnectRequest {
    pub accessToken: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedConnectResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BanUserRequest {
    pub userPublicId: Nanoid<16, Base62Alphabet>,
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BanUserResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppConfigRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub callBackUrl: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppConfigResponse {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub createdAt: i64,
    pub appApiKey: String,
    pub callbackApiKey: String,
    pub minPasswordLength: i32,
    pub requiredPasswordChars: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppKeyRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub label: String,
    #[serde(default)]
    pub expiresInSecs: Option<i64>,
    pub createdBy: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppKeyResponse {
    pub keyId: String,
    pub key: String,
    pub label: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAppConfigRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAppConfigResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub userPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditAppConfigRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    #[serde(default)]
    pub callBackUrl: Option<String>,
    #[serde(default)]
    pub minPasswordLength: Option<i32>,
    #[serde(default)]
    pub requiredPasswordChars: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditAppConfigResponse {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub callBackUrl: String,
    pub minPasswordLength: i32,
    pub requiredPasswordChars: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAppSecurityRulesRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAppSecurityRulesResponse {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub minPasswordLength: i32,
    pub requiredPasswordChars: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserSecurityRequest {
    pub userPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserSecurityResponse {
    pub totpEnabled: bool,
    #[serde(default)]
    pub telegramUsername: Option<String>,
    pub telegramConfirmed: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InspectAppCredentialRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub appKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InspectAppCredentialResponse {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub keyId: String,
    pub service: bool,
    #[serde(default)]
    pub expiresAt: Option<i64>,
    #[serde(default)]
    pub revokedAt: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListAppKeysRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListAppKeysResponse {
    pub keys: Vec<AppKeyInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlatformConnectRequest {
    pub platformApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlatformConnectResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicConnectRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicConnectResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTokenRequest {
    pub token: String,
    pub username: String,
    pub userPubId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTokenResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveUserDeletedRequest {
    pub userPubId: Nanoid<16, Base62Alphabet>,
    #[serde(default)]
    pub appPubId: Option<Nanoid<16, Base62Alphabet>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveUserDeletedResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveUserInfoRequest {
    pub userPubId: Nanoid<16, Base62Alphabet>,
    pub username: String,
    #[serde(default)]
    pub appPubId: Option<Nanoid<16, Base62Alphabet>>,
    #[serde(default)]
    pub token: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveUserInfoResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateAppApiKeyRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateAppApiKeyResponse {
    pub appApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RevokeAppKeyRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub keyId: String,
    pub graceSecs: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RevokeAppKeyResponse {
    pub keyId: String,
    #[serde(default)]
    pub expiresAt: Option<i64>,
    #[serde(default)]
    pub revokedAt: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RotateAppCallbackCredentialRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RotateAppCallbackCredentialResponse {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub callbackApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAppServiceRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub service: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAppServiceResponse {
    pub service: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetLogLevelRequest {
    #[serde(default)]
    pub logLevel: Option<LogLevel>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetLogLevelResponse {
    pub logLevel: LogLevel,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupResponse {
    pub accessToken: String,
    pub encryptionKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitPasswordRequest {
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitPasswordResponse {
    pub accessToken: String,
    pub encryptionKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitUsernameRequest {
    pub appPublicId: Nanoid<16, Base62Alphabet>,
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitUsernameResponse {
    pub expiresAt: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnbanUserRequest {
    pub userPublicId: Nanoid<16, Base62Alphabet>,
    pub appPublicId: Nanoid<16, Base62Alphabet>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnbanUserResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTokenRequest {
    pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTokenResponse {
    pub valid: bool,
    #[serde(default)]
    pub userPubId: Option<Nanoid<16, Base62Alphabet>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignupError {
    /// Invalid username
    InvalidUsername,
    /// App not found
    AppNotFound,
    /// Account is disabled or banned
    AccountForbidden,
    /// Invalid password
    InvalidPassword,
    /// App callback failed
    CallbackFailed,
}

impl From<SignupError> for CustomError {
    fn from(err: SignupError) -> Self {
        match err {
            SignupError::InvalidUsername => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Invalid username")
                .with_kind("InvalidUsername"),
            SignupError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            SignupError::AccountForbidden => CustomError::new(EnumErrorCode::Forbidden)
                .with_message("Account is disabled or banned")
                .with_kind("AccountForbidden"),
            SignupError::InvalidPassword => CustomError::new(EnumErrorCode::Unauthorized)
                .with_message("Invalid password")
                .with_kind("InvalidPassword"),
            SignupError::CallbackFailed => CustomError::new(EnumErrorCode::BadGateway)
                .with_message("App callback failed")
                .with_kind("CallbackFailed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubmitUsernameError {
    /// App not found
    AppNotFound,
    /// User not found
    UserNotFound,
    /// Account is disabled or banned
    AccountForbidden,
}

impl From<SubmitUsernameError> for CustomError {
    fn from(err: SubmitUsernameError) -> Self {
        match err {
            SubmitUsernameError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            SubmitUsernameError::UserNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("User not found")
                .with_kind("UserNotFound"),
            SubmitUsernameError::AccountForbidden => CustomError::new(EnumErrorCode::Forbidden)
                .with_message("Account is disabled or banned")
                .with_kind("AccountForbidden"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubmitPasswordError {
    /// Call SubmitUsername before SubmitPassword
    AuthFlowRequired,
    /// Invalid password
    InvalidPassword,
    /// Access denied to this app
    AccessDenied,
    /// App callback failed
    CallbackFailed,
}

impl From<SubmitPasswordError> for CustomError {
    fn from(err: SubmitPasswordError) -> Self {
        match err {
            SubmitPasswordError::AuthFlowRequired => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Call SubmitUsername before SubmitPassword")
                .with_kind("AuthFlowRequired"),
            SubmitPasswordError::InvalidPassword => CustomError::new(EnumErrorCode::Unauthorized)
                .with_message("Invalid password")
                .with_kind("InvalidPassword"),
            SubmitPasswordError::AccessDenied => CustomError::new(EnumErrorCode::Forbidden)
                .with_message("Access denied to this app")
                .with_kind("AccessDenied"),
            SubmitPasswordError::CallbackFailed => CustomError::new(EnumErrorCode::BadGateway)
                .with_message("App callback failed")
                .with_kind("CallbackFailed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlatformConnectError {
    /// Wrong platformApiKey
    InvalidApiKey,
}

impl From<PlatformConnectError> for CustomError {
    fn from(err: PlatformConnectError) -> Self {
        match err {
            PlatformConnectError::InvalidApiKey => CustomError::new(EnumErrorCode::Unauthorized)
                .with_message("Wrong platformApiKey")
                .with_kind("InvalidApiKey"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateAppConfigError {
    /// Failed to create app configuration
    InternalError,
}

impl From<CreateAppConfigError> for CustomError {
    fn from(err: CreateAppConfigError) -> Self {
        match err {
            CreateAppConfigError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to create app configuration")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BanUserError {
    /// User not found
    UserNotFound,
    /// App not found
    AppNotFound,
    /// Membership not found
    MembershipNotFound,
}

impl From<BanUserError> for CustomError {
    fn from(err: BanUserError) -> Self {
        match err {
            BanUserError::UserNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("User not found")
                .with_kind("UserNotFound"),
            BanUserError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            BanUserError::MembershipNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("Membership not found")
                .with_kind("MembershipNotFound"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnbanUserError {
    /// User not found
    UserNotFound,
    /// App not found
    AppNotFound,
    /// Membership not found
    MembershipNotFound,
}

impl From<UnbanUserError> for CustomError {
    fn from(err: UnbanUserError) -> Self {
        match err {
            UnbanUserError::UserNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("User not found")
                .with_kind("UserNotFound"),
            UnbanUserError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            UnbanUserError::MembershipNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("Membership not found")
                .with_kind("MembershipNotFound"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteUserError {
    /// Failed to delete user
    InternalError,
}

impl From<DeleteUserError> for CustomError {
    fn from(err: DeleteUserError) -> Self {
        match err {
            DeleteUserError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to delete user")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteAppConfigError {
    /// Failed to delete app configuration
    InternalError,
}

impl From<DeleteAppConfigError> for CustomError {
    fn from(err: DeleteAppConfigError) -> Self {
        match err {
            DeleteAppConfigError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to delete app configuration")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EditAppConfigError {
    /// App not found
    AppNotFound,
    /// Failed to edit app configuration
    InternalError,
}

impl From<EditAppConfigError> for CustomError {
    fn from(err: EditAppConfigError) -> Self {
        match err {
            EditAppConfigError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            EditAppConfigError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to edit app configuration")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetAppSecurityRulesError {
    /// App not found
    AppNotFound,
}

impl From<GetAppSecurityRulesError> for CustomError {
    fn from(err: GetAppSecurityRulesError) -> Self {
        match err {
            GetAppSecurityRulesError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SetLogLevelError {
    /// Failed to set log level
    InvalidLogLevel,
}

impl From<SetLogLevelError> for CustomError {
    fn from(err: SetLogLevelError) -> Self {
        match err {
            SetLogLevelError::InvalidLogLevel => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Failed to set log level")
                .with_kind("InvalidLogLevel"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RegenerateAppApiKeyError {
    /// App not found
    AppNotFound,
    /// Failed to regenerate app API key
    InternalError,
}

impl From<RegenerateAppApiKeyError> for CustomError {
    fn from(err: RegenerateAppApiKeyError) -> Self {
        match err {
            RegenerateAppApiKeyError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            RegenerateAppApiKeyError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to regenerate app API key")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetUserSecurityError {
    /// User not found
    UserNotFound,
}

impl From<GetUserSecurityError> for CustomError {
    fn from(err: GetUserSecurityError) -> Self {
        match err {
            GetUserSecurityError::UserNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("User not found")
                .with_kind("UserNotFound"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateAppKeyError {
    /// App not found
    AppNotFound,
    /// Invalid app key label or expiry
    InvalidKeyRequest,
    /// Failed to create app key
    InternalError,
}

impl From<CreateAppKeyError> for CustomError {
    fn from(err: CreateAppKeyError) -> Self {
        match err {
            CreateAppKeyError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            CreateAppKeyError::InvalidKeyRequest => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Invalid app key label or expiry")
                .with_kind("InvalidKeyRequest"),
            CreateAppKeyError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to create app key")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ListAppKeysError {
    /// App not found
    AppNotFound,
}

impl From<ListAppKeysError> for CustomError {
    fn from(err: ListAppKeysError) -> Self {
        match err {
            ListAppKeysError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RevokeAppKeyError {
    /// App or app key not found
    AppNotFound,
    /// Grace period must be non-negative
    InvalidGracePeriod,
    /// Failed to revoke app key
    InternalError,
}

impl From<RevokeAppKeyError> for CustomError {
    fn from(err: RevokeAppKeyError) -> Self {
        match err {
            RevokeAppKeyError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App or app key not found")
                .with_kind("AppNotFound"),
            RevokeAppKeyError::InvalidGracePeriod => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Grace period must be non-negative")
                .with_kind("InvalidGracePeriod"),
            RevokeAppKeyError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to revoke app key")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RotateAppCallbackCredentialError {
    /// App not found
    AppNotFound,
    /// Failed to regenerate callback credential
    InternalError,
}

impl From<RotateAppCallbackCredentialError> for CustomError {
    fn from(err: RotateAppCallbackCredentialError) -> Self {
        match err {
            RotateAppCallbackCredentialError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            RotateAppCallbackCredentialError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to regenerate callback credential")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InspectAppCredentialError {
    /// Only service apps may inspect app credentials
    AppNotService,
    /// Invalid app credential
    InvalidCredential,
    /// Failed to inspect app credential
    InternalError,
}

impl From<InspectAppCredentialError> for CustomError {
    fn from(err: InspectAppCredentialError) -> Self {
        match err {
            InspectAppCredentialError::AppNotService => CustomError::new(EnumErrorCode::Forbidden)
                .with_message("Only service apps may inspect app credentials")
                .with_kind("AppNotService"),
            InspectAppCredentialError::InvalidCredential => CustomError::new(EnumErrorCode::Unauthorized)
                .with_message("Invalid app credential")
                .with_kind("InvalidCredential"),
            InspectAppCredentialError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to inspect app credential")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SetAppServiceError {
    /// App not found
    AppNotFound,
    /// Failed to update app service capability
    InternalError,
}

impl From<SetAppServiceError> for CustomError {
    fn from(err: SetAppServiceError) -> Self {
        match err {
            SetAppServiceError::AppNotFound => CustomError::new(EnumErrorCode::NotFound)
                .with_message("App not found")
                .with_kind("AppNotFound"),
            SetAppServiceError::InternalError => CustomError::new(EnumErrorCode::InternalError)
                .with_message("Failed to update app service capability")
                .with_kind("InternalError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiKeyConnectError {
    /// Wrong appApiKey
    InvalidApiKey,
}

impl From<ApiKeyConnectError> for CustomError {
    fn from(err: ApiKeyConnectError) -> Self {
        match err {
            ApiKeyConnectError::InvalidApiKey => CustomError::new(EnumErrorCode::Unauthorized)
                .with_message("Wrong appApiKey")
                .with_kind("InvalidApiKey"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthorizedConnectError {
    /// Wrong accessToken
    InvalidAccessToken,
}

impl From<AuthorizedConnectError> for CustomError {
    fn from(err: AuthorizedConnectError) -> Self {
        match err {
            AuthorizedConnectError::InvalidAccessToken => CustomError::new(EnumErrorCode::Unauthorized)
                .with_message("Wrong accessToken")
                .with_kind("InvalidAccessToken"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReceiveTokenError {
    /// Invalid token
    InvalidToken,
}

impl From<ReceiveTokenError> for CustomError {
    fn from(err: ReceiveTokenError) -> Self {
        match err {
            ReceiveTokenError::InvalidToken => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Invalid token")
                .with_kind("InvalidToken"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReceiveUserInfoError {
    /// Invalid token
    InvalidToken,
}

impl From<ReceiveUserInfoError> for CustomError {
    fn from(err: ReceiveUserInfoError) -> Self {
        match err {
            ReceiveUserInfoError::InvalidToken => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Invalid token")
                .with_kind("InvalidToken"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ValidateTokenError {
    /// Invalid token
    InvalidToken,
}

impl From<ValidateTokenError> for CustomError {
    fn from(err: ValidateTokenError) -> Self {
        match err {
            ValidateTokenError::InvalidToken => CustomError::new(EnumErrorCode::BadRequest)
                .with_message("Invalid token")
                .with_kind("InvalidToken"),
        }
    }
}

impl WsRequest for PublicConnectRequest {
    type Response = PublicConnectResponse;
    const METHOD_ID: u32 = 0;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "PublicConnect",
  "code": 0,
  "parameters": [],
  "returns": [],
  "stream_response": null,
  "description": "Initiates a websocket connection session with that permits access to endpoints with the UserRole::Public role",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": []
}"#;
}
impl WsResponse for PublicConnectResponse {
    type Request = PublicConnectRequest;
}

impl WsRequest for SignupRequest {
    type Response = SignupResponse;
    const METHOD_ID: u32 = 10;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "Signup",
  "code": 10,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "username",
      "ty": "String"
    },
    {
      "name": "password",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "accessToken",
      "ty": "String"
    },
    {
      "name": "encryptionKey",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Frontend creates new user account.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": [
    {
      "name": "InvalidUsername",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Invalid username",
      "fields": []
    },
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "AccountForbidden",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Forbidden"
      },
      "message": "Account is disabled or banned",
      "fields": []
    },
    {
      "name": "InvalidPassword",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Unauthorized"
      },
      "message": "Invalid password",
      "fields": []
    },
    {
      "name": "CallbackFailed",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadGateway"
      },
      "message": "App callback failed",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for SignupResponse {
    type Request = SignupRequest;
}

impl WsRequest for SubmitUsernameRequest {
    type Response = SubmitUsernameResponse;
    const METHOD_ID: u32 = 12;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "SubmitUsername",
  "code": 12,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "username",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "expiresAt",
      "ty": "Int64"
    }
  ],
  "stream_response": null,
  "description": "Step 1: Frontend submits username during auth flow.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "UserNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "User not found",
      "fields": []
    },
    {
      "name": "AccountForbidden",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Forbidden"
      },
      "message": "Account is disabled or banned",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for SubmitUsernameResponse {
    type Request = SubmitUsernameRequest;
}

impl WsRequest for SubmitPasswordRequest {
    type Response = SubmitPasswordResponse;
    const METHOD_ID: u32 = 13;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "SubmitPassword",
  "code": 13,
  "parameters": [
    {
      "name": "password",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "accessToken",
      "ty": "String"
    },
    {
      "name": "encryptionKey",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": [
    {
      "name": "AuthFlowRequired",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Call SubmitUsername before SubmitPassword",
      "fields": []
    },
    {
      "name": "InvalidPassword",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Unauthorized"
      },
      "message": "Invalid password",
      "fields": []
    },
    {
      "name": "AccessDenied",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Forbidden"
      },
      "message": "Access denied to this app",
      "fields": []
    },
    {
      "name": "CallbackFailed",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadGateway"
      },
      "message": "App callback failed",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for SubmitPasswordResponse {
    type Request = SubmitPasswordRequest;
}

impl WsRequest for PlatformConnectRequest {
    type Response = PlatformConnectResponse;
    const METHOD_ID: u32 = 100;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "PlatformConnect",
  "code": 100,
  "parameters": [
    {
      "name": "platformApiKey",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Handles platform API Key login to initiate the connection session between Honey API Backend and this server",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": [
    {
      "name": "InvalidApiKey",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Unauthorized"
      },
      "message": "Wrong platformApiKey",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for PlatformConnectResponse {
    type Request = PlatformConnectRequest;
}

impl WsRequest for CreateAppConfigRequest {
    type Response = CreateAppConfigResponse;
    const METHOD_ID: u32 = 111;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "CreateAppConfig",
  "code": 111,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "callBackUrl",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "createdAt",
      "ty": "Int64"
    },
    {
      "name": "appApiKey",
      "ty": "String"
    },
    {
      "name": "callbackApiKey",
      "ty": "String"
    },
    {
      "name": "minPasswordLength",
      "ty": "Int32"
    },
    {
      "name": "requiredPasswordChars",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Platform creates an app and returns its app key and separate callback credential once",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to create app configuration",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for CreateAppConfigResponse {
    type Request = CreateAppConfigRequest;
}

impl WsRequest for BanUserRequest {
    type Response = BanUserResponse;
    const METHOD_ID: u32 = 112;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "BanUser",
  "code": 112,
  "parameters": [
    {
      "name": "userPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Ban a user from provided app",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "UserNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "User not found",
      "fields": []
    },
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "MembershipNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "Membership not found",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for BanUserResponse {
    type Request = BanUserRequest;
}

impl WsRequest for UnbanUserRequest {
    type Response = UnbanUserResponse;
    const METHOD_ID: u32 = 113;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "UnbanUser",
  "code": 113,
  "parameters": [
    {
      "name": "userPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Unban a user from a specific app",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "UserNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "User not found",
      "fields": []
    },
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "MembershipNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "Membership not found",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for UnbanUserResponse {
    type Request = UnbanUserRequest;
}

impl WsRequest for DeleteUserRequest {
    type Response = DeleteUserResponse;
    const METHOD_ID: u32 = 114;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "DeleteUser",
  "code": 114,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "userPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Delete a user",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to delete user",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for DeleteUserResponse {
    type Request = DeleteUserRequest;
}

impl WsRequest for DeleteAppConfigRequest {
    type Response = DeleteAppConfigResponse;
    const METHOD_ID: u32 = 115;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "DeleteAppConfig",
  "code": 115,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Delete app configuration",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to delete app configuration",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for DeleteAppConfigResponse {
    type Request = DeleteAppConfigRequest;
}

impl WsRequest for EditAppConfigRequest {
    type Response = EditAppConfigResponse;
    const METHOD_ID: u32 = 116;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "EditAppConfig",
  "code": 116,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "callBackUrl",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "minPasswordLength",
      "ty": {
        "Optional": "Int32"
      }
    },
    {
      "name": "requiredPasswordChars",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "callBackUrl",
      "ty": "String"
    },
    {
      "name": "minPasswordLength",
      "ty": "Int32"
    },
    {
      "name": "requiredPasswordChars",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Edit app configuration",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to edit app configuration",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for EditAppConfigResponse {
    type Request = EditAppConfigRequest;
}

impl WsRequest for GetAppSecurityRulesRequest {
    type Response = GetAppSecurityRulesResponse;
    const METHOD_ID: u32 = 117;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "GetAppSecurityRules",
  "code": 117,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "minPasswordLength",
      "ty": "Int32"
    },
    {
      "name": "requiredPasswordChars",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Get security rules contained within current app's configuration",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for GetAppSecurityRulesResponse {
    type Request = GetAppSecurityRulesRequest;
}

impl WsRequest for SetLogLevelRequest {
    type Response = SetLogLevelResponse;
    const METHOD_ID: u32 = 118;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "SetLogLevel",
  "code": 118,
  "parameters": [
    {
      "name": "logLevel",
      "ty": {
        "Optional": {
          "EnumRef": {
            "name": "LogLevel"
          }
        }
      }
    }
  ],
  "returns": [
    {
      "name": "logLevel",
      "ty": {
        "EnumRef": {
          "name": "LogLevel"
        }
      }
    }
  ],
  "stream_response": null,
  "description": "Set log level at runtime",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "InvalidLogLevel",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Failed to set log level",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for SetLogLevelResponse {
    type Request = SetLogLevelRequest;
}

impl WsRequest for RegenerateAppApiKeyRequest {
    type Response = RegenerateAppApiKeyResponse;
    const METHOD_ID: u32 = 119;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "RegenerateAppApiKey",
  "code": 119,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [
    {
      "name": "appApiKey",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Compatibility rotation: creates a new app key and immediately revokes every other app key",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to regenerate app API key",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for RegenerateAppApiKeyResponse {
    type Request = RegenerateAppApiKeyRequest;
}

impl WsRequest for GetUserSecurityRequest {
    type Response = GetUserSecurityResponse;
    const METHOD_ID: u32 = 120;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "GetUserSecurity",
  "code": 120,
  "parameters": [
    {
      "name": "userPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [
    {
      "name": "totpEnabled",
      "ty": "Boolean"
    },
    {
      "name": "telegramUsername",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "telegramConfirmed",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "A user's second factors: whether an authenticator app is enrolled, and the Telegram account bound for recovery, if any",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "UserNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "User not found",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for GetUserSecurityResponse {
    type Request = GetUserSecurityRequest;
}

impl WsRequest for CreateAppKeyRequest {
    type Response = CreateAppKeyResponse;
    const METHOD_ID: u32 = 121;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "CreateAppKey",
  "code": 121,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "label",
      "ty": "String"
    },
    {
      "name": "expiresInSecs",
      "ty": {
        "Optional": "Int64"
      }
    },
    {
      "name": "createdBy",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "keyId",
      "ty": "String"
    },
    {
      "name": "key",
      "ty": "String"
    },
    {
      "name": "label",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Creates a hashed app key and returns its key string once",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "InvalidKeyRequest",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Invalid app key label or expiry",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to create app key",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for CreateAppKeyResponse {
    type Request = CreateAppKeyRequest;
}

impl WsRequest for ListAppKeysRequest {
    type Response = ListAppKeysResponse;
    const METHOD_ID: u32 = 122;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "ListAppKeys",
  "code": 122,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [
    {
      "name": "keys",
      "ty": {
        "StructTable": {
          "struct_ref": "AppKeyInfo"
        }
      }
    }
  ],
  "stream_response": null,
  "description": "Lists app key metadata without secrets or hashes",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for ListAppKeysResponse {
    type Request = ListAppKeysRequest;
}

impl WsRequest for RevokeAppKeyRequest {
    type Response = RevokeAppKeyResponse;
    const METHOD_ID: u32 = 123;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "RevokeAppKey",
  "code": 123,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "keyId",
      "ty": "String"
    },
    {
      "name": "graceSecs",
      "ty": "Int64"
    }
  ],
  "returns": [
    {
      "name": "keyId",
      "ty": "String"
    },
    {
      "name": "expiresAt",
      "ty": {
        "Optional": "Int64"
      }
    },
    {
      "name": "revokedAt",
      "ty": {
        "Optional": "Int64"
      }
    }
  ],
  "stream_response": null,
  "description": "Revokes an app key now or schedules its expiry after a grace period",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App or app key not found",
      "fields": []
    },
    {
      "name": "InvalidGracePeriod",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Grace period must be non-negative",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to revoke app key",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for RevokeAppKeyResponse {
    type Request = RevokeAppKeyRequest;
}

impl WsRequest for RotateAppCallbackCredentialRequest {
    type Response = RotateAppCallbackCredentialResponse;
    const METHOD_ID: u32 = 124;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "RotateAppCallbackCredential",
  "code": 124,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "callbackApiKey",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Creates and returns a separate callback credential once",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to regenerate callback credential",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for RotateAppCallbackCredentialResponse {
    type Request = RotateAppCallbackCredentialRequest;
}

impl WsRequest for InspectAppCredentialRequest {
    type Response = InspectAppCredentialResponse;
    const METHOD_ID: u32 = 125;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "InspectAppCredential",
  "code": 125,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "appKey",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "keyId",
      "ty": "String"
    },
    {
      "name": "service",
      "ty": "Boolean"
    },
    {
      "name": "expiresAt",
      "ty": {
        "Optional": "Int64"
      }
    },
    {
      "name": "revokedAt",
      "ty": {
        "Optional": "Int64"
      }
    }
  ],
  "stream_response": null,
  "description": "Proves an app credential and returns its key status metadata to the trusted API backend",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ],
  "errors": [
    {
      "name": "AppNotService",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Forbidden"
      },
      "message": "Only service apps may inspect app credentials",
      "fields": []
    },
    {
      "name": "InvalidCredential",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Unauthorized"
      },
      "message": "Invalid app credential",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to inspect app credential",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for InspectAppCredentialResponse {
    type Request = InspectAppCredentialRequest;
}

impl WsRequest for SetAppServiceRequest {
    type Response = SetAppServiceResponse;
    const METHOD_ID: u32 = 126;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "SetAppService",
  "code": 126,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "service",
      "ty": "Boolean"
    }
  ],
  "returns": [
    {
      "name": "service",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Sets whether an app may act as a service verifier client",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ],
  "errors": [
    {
      "name": "AppNotFound",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "NotFound"
      },
      "message": "App not found",
      "fields": []
    },
    {
      "name": "InternalError",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "InternalError"
      },
      "message": "Failed to update app service capability",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for SetAppServiceResponse {
    type Request = SetAppServiceRequest;
}

impl WsRequest for ApiKeyConnectRequest {
    type Response = ApiKeyConnectResponse;
    const METHOD_ID: u32 = 200;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "ApiKeyConnect",
  "code": 200,
  "parameters": [
    {
      "name": "appApiKey",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Authenticates Honey Auth callbacks using the app's separate callback credential",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": [
    {
      "name": "InvalidApiKey",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Unauthorized"
      },
      "message": "Wrong appApiKey",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for ApiKeyConnectResponse {
    type Request = ApiKeyConnectRequest;
}

impl WsRequest for AuthorizedConnectRequest {
    type Response = AuthorizedConnectResponse;
    const METHOD_ID: u32 = 201;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "AuthorizedConnect",
  "code": 201,
  "parameters": [
    {
      "name": "accessToken",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "A user connects to an app with the access token auth issued them.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ],
  "errors": [
    {
      "name": "InvalidAccessToken",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "Unauthorized"
      },
      "message": "Wrong accessToken",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for AuthorizedConnectResponse {
    type Request = AuthorizedConnectRequest;
}

impl WsRequest for ReceiveTokenRequest {
    type Response = ReceiveTokenResponse;
    const METHOD_ID: u32 = 210;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "ReceiveToken",
  "code": 210,
  "parameters": [
    {
      "name": "token",
      "ty": "String"
    },
    {
      "name": "username",
      "ty": "String"
    },
    {
      "name": "userPubId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Backend receives auth tokens, happens after login",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ],
  "errors": [
    {
      "name": "InvalidToken",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Invalid token",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for ReceiveTokenResponse {
    type Request = ReceiveTokenRequest;
}

impl WsRequest for ReceiveUserInfoRequest {
    type Response = ReceiveUserInfoResponse;
    const METHOD_ID: u32 = 211;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "ReceiveUserInfo",
  "code": 211,
  "parameters": [
    {
      "name": "userPubId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "username",
      "ty": "String"
    },
    {
      "name": "appPubId",
      "ty": {
        "Optional": {
          "NanoId": {
            "len": 16
          }
        }
      }
    },
    {
      "name": "token",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Backend receives user info with optional token, happens after new user signs up. Platform app also receives this so that it can maintain records of app users, in which case Token will be set to None",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ],
  "errors": [
    {
      "name": "InvalidToken",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Invalid token",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for ReceiveUserInfoResponse {
    type Request = ReceiveUserInfoRequest;
}

impl WsRequest for ReceiveUserDeletedRequest {
    type Response = ReceiveUserDeletedResponse;
    const METHOD_ID: u32 = 212;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "ReceiveUserDeleted",
  "code": 212,
  "parameters": [
    {
      "name": "userPubId",
      "ty": {
        "NanoId": {
          "len": 16
        }
      }
    },
    {
      "name": "appPubId",
      "ty": {
        "Optional": {
          "NanoId": {
            "len": 16
          }
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Backend receives notification when a user is deleted or banned. App should clean up all user data and invalidate tokens.",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ],
  "errors": []
}"#;
}
impl WsResponse for ReceiveUserDeletedResponse {
    type Request = ReceiveUserDeletedRequest;
}

impl WsRequest for ValidateTokenRequest {
    type Response = ValidateTokenResponse;
    const METHOD_ID: u32 = 213;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "ValidateToken",
  "code": 213,
  "parameters": [
    {
      "name": "token",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "valid",
      "ty": "Boolean"
    },
    {
      "name": "userPubId",
      "ty": {
        "Optional": {
          "NanoId": {
            "len": 16
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "App validates an existing token and returns whether it is valid along with the associated userPubId",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ],
  "errors": [
    {
      "name": "InvalidToken",
      "code": {
        "ty": {
          "EnumRef": {
            "name": "ErrorCode"
          }
        },
        "variant": "BadRequest"
      },
      "message": "Invalid token",
      "fields": []
    }
  ]
}"#;
}
impl WsResponse for ValidateTokenResponse {
    type Request = ValidateTokenRequest;
}
