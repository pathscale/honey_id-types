use endpoint_libs::libs::error_code::ErrorCode;
use endpoint_libs::libs::types::*;
use endpoint_libs::libs::ws::toolbox::CustomError;
use endpoint_libs::libs::ws::*;
use num_derive::FromPrimitive;
use psc_nanoid::{Nanoid, alphabet::Base62Alphabet};
use rkyv::Archive;
use serde::*;
use std::net::IpAddr;
use strum_macros::{Display, EnumString};
use uuid::Uuid;
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

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, Hash,
)]
pub enum EnumEndpoint {
    ///
    PublicConnect = 0,
    ///
    Signup = 10,
    ///
    SubmitUsername = 12,
    ///
    SubmitPassword = 13,
    ///
    PlatformConnect = 100,
    ///
    CreateAppConfig = 111,
    ///
    BanUser = 112,
    ///
    UnbanUser = 113,
    ///
    DeleteUser = 114,
    ///
    DeleteAppConfig = 115,
    ///
    EditAppConfig = 116,
    ///
    GetAppSecurityRules = 117,
    ///
    SetLogLevel = 118,
    ///
    ApiKeyConnect = 200,
    ///
    AuthorizedConnect = 201,
    ///
    ReceiveToken = 210,
    ///
    ReceiveUserInfo = 211,
    ///
    ReceiveUserDeleted = 212,
    ///
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
    pub minPasswordLength: i32,
    pub requiredPasswordChars: String,
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
      "name": "minPasswordLength",
      "ty": "Int32"
    },
    {
      "name": "requiredPasswordChars",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Platform can create new apps",
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
  "description": "",
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
  "description": "",
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
