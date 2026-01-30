use endpoint_libs::libs::error_code::ErrorCode;
use endpoint_libs::libs::types::*;
use endpoint_libs::libs::ws::*;
use num_derive::FromPrimitive;
use serde::*;
use strum_macros::{Display, EnumString};
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
pub enum EnumUserRole {
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
}
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
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
    ApiKeyConnect = 100,
    ///
    GetAppPublicId = 101,
    ///
    ReceiveToken = 110,
}

impl EnumEndpoint {
    pub fn schema(&self) -> endpoint_libs::model::EndpointSchema {
        let schema = match self {
            Self::PublicConnect => PublicConnectRequest::SCHEMA,
            Self::Signup => SignupRequest::SCHEMA,
            Self::SubmitUsername => SubmitUsernameRequest::SCHEMA,
            Self::SubmitPassword => SubmitPasswordRequest::SCHEMA,
            Self::ApiKeyConnect => ApiKeyConnectRequest::SCHEMA,
            Self::GetAppPublicId => GetAppPublicIdRequest::SCHEMA,
            Self::ReceiveToken => ReceiveTokenRequest::SCHEMA,
        };
        serde_json::from_str(schema).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBadRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalServerError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotImplemented {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDatabaseError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidService {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserForbidden {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserRoleForbidden {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreeTos {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreePrivacyPolicy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNoAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserInvalidAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserInvalidPassword {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDuplicateRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidExpression {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidArgument {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidState {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidSeq {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidMethod {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProtocolViolation {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRestrictedUserPrivileges {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRole {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalError {}
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
pub enum EnumErrorCode {
    /// Custom Bad Request
    BadRequest = 100400,
    /// Custom NotFoundResource
    NotFound = 100404,
    /// Custom Internal Server Error
    InternalServerError = 100500,
    /// Custom Method not implemented
    NotImplemented = 100501,
    /// Custom Database error
    DatabaseError = 100601,
    /// Custom Invalid Service
    InvalidService = 100602,
    /// Custom Forbidden user
    UserForbidden = 101403,
    /// Custom User not found
    UserNotFound = 101404,
    /// Custom Must agree to the terms of service
    UserMustAgreeTos = 101601,
    /// Custom Must agree to the privacy policy
    UserMustAgreePrivacyPolicy = 101602,
    /// Custom No auth token
    UserNoAuthToken = 101604,
    /// Custom token invalid
    UserInvalidAuthToken = 101605,
    /// Custom password invalid
    UserInvalidPassword = 101606,
    /// Custom Insufficient role for user
    UserRoleForbidden = 102403,
    /// Custom Duplicate request
    DuplicateRequest = 103001,
    /// Custom Invalid expression
    InvalidExpression = 104000,
    /// SQL R0001 InvalidArgument
    InvalidArgument = 45349633,
    /// SQL R0002 InvalidState
    InvalidState = 45349634,
    /// SQL R0003 InvalidSeq
    InvalidSeq = 45349635,
    /// SQL R0004 InvalidMethod
    InvalidMethod = 45349636,
    /// SQL R0005 ProtocolViolation
    ProtocolViolation = 45349637,
    /// SQL R000N RestrictedUserPrivileges
    RestrictedUserPrivileges = 45349655,
    /// SQL R000S InvalidRole
    InvalidRole = 45349660,
    /// SQL R001G InternalError
    InternalError = 45349684,
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
pub struct GetAppPublicIdRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAppPublicIdResponse {
    pub appPublicId: uuid::Uuid,
}
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
    pub userPubId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTokenResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub appPublicId: uuid::Uuid,
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupResponse {
    pub accessToken: String,
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
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitUsernameRequest {
    pub appPublicId: uuid::Uuid,
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitUsernameResponse {
    pub expiresAt: i64,
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
  ]
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
      "ty": "UUID"
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
    }
  ],
  "stream_response": null,
  "description": "Frontend creates new user account.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
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
      "ty": "UUID"
    },
    {
      "name": "username",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "expiresAt",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "Step 1: Frontend submits username during auth flow.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
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
    }
  ],
  "stream_response": null,
  "description": "Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for SubmitPasswordResponse {
    type Request = SubmitPasswordRequest;
}

impl WsRequest for ApiKeyConnectRequest {
    type Response = ApiKeyConnectResponse;
    const METHOD_ID: u32 = 100;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "ApiKeyConnect",
  "code": 100,
  "parameters": [
    {
      "name": "appApiKey",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Handles API Key login to initiate the AppApiKey connection session between App Backend and Honey Auth Server",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for ApiKeyConnectResponse {
    type Request = ApiKeyConnectRequest;
}

impl WsRequest for GetAppPublicIdRequest {
    type Response = GetAppPublicIdResponse;
    const METHOD_ID: u32 = 101;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "GetAppPublicId",
  "code": 101,
  "parameters": [],
  "returns": [
    {
      "name": "appPublicId",
      "ty": "UUID"
    }
  ],
  "stream_response": null,
  "description": "Returns `appPublicId` which will be used for auth flows",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for GetAppPublicIdResponse {
    type Request = GetAppPublicIdRequest;
}

impl WsRequest for ReceiveTokenRequest {
    type Response = ReceiveTokenResponse;
    const METHOD_ID: u32 = 110;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "ReceiveToken",
  "code": 110,
  "parameters": [
    {
      "name": "token",
      "ty": "String"
    },
    {
      "name": "userPubId",
      "ty": "UUID"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Backend receives HoneyAuth tokens.",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ]
}"#;
}
impl WsResponse for ReceiveTokenResponse {
    type Request = ReceiveTokenRequest;
}
