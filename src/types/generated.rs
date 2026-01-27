#![allow(unused_imports, dead_code)]

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
pub enum EnumAppBillingStatus {
    /// Free tier.
    PayRequired = 0,
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
pub enum EnumAppSubscriptionStatus {
    /// WIP
    Active = 0,
    /// WIP
    Cancelled = 1,
    /// WIP
    ScheduledForCancel = 2,
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
pub enum EnumEventSeverity {
    /// Represents a message about successful action.
    Success = 0,
    /// Represents a message about some progress.
    Info = 1,
    /// Represents a message that requires attention.
    Warn = 2,
    /// Represents a message that marks a failure.
    Error = 3,
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
pub enum EnumExternalUserStatus {
    /// Active user.
    Enabled = 1,
    /// Inactive user.
    Disabled = 2,
    /// Banned user.
    Banned = 3,
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
pub enum EnumInternalUserStatus {
    /// Active user.
    Enabled = 1,
    /// Inactive user.
    Disabled = 2,
    /// Banned user.
    Banned = 3,
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
pub enum EnumAppStatus {
    /// Active application.
    Enabled = 1,
    /// Inactive user.
    Disabled = 2,
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
pub enum EnumPaymentStatus {
    /// Payment is pending.
    Pending = 0,
    /// Payment has expired.
    Expired = 1,
    /// Payment has been cancelled.
    Cancelled = 2,
    /// Payment has been completed.
    Completed = 3,
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
pub enum EnumRoleInApp {
    /// Admin in application.
    Admin = 1,
    /// Member in application.
    Member = 2,
    /// Guest in application.
    Guest = 3,
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
    StartAuth = 10,
    ///
    Signup = 11,
    ///
    SubmitUsername = 12,
    ///
    SubmitPassword = 13,
    ///
    RefreshTokenExchange = 14,
    ///
    TokenRevoke = 15,
    ///
    AddWaitlistLead = 20,
    ///
    KardAddWaitlistLead = 30,
    ///
    ApiKeyConnect = 100,
    ///
    TokenIntrospect = 110,
    ///
    SubscribeTokenRevocations = 111,
}

impl EnumEndpoint {
    pub fn schema(&self) -> endpoint_libs::model::EndpointSchema {
        let schema = match self {
            Self::PublicConnect => PublicConnectRequest::SCHEMA,
            Self::StartAuth => StartAuthRequest::SCHEMA,
            Self::Signup => SignupRequest::SCHEMA,
            Self::SubmitUsername => SubmitUsernameRequest::SCHEMA,
            Self::SubmitPassword => SubmitPasswordRequest::SCHEMA,
            Self::RefreshTokenExchange => RefreshTokenExchangeRequest::SCHEMA,
            Self::TokenRevoke => TokenRevokeRequest::SCHEMA,
            Self::AddWaitlistLead => AddWaitlistLeadRequest::SCHEMA,
            Self::KardAddWaitlistLead => KardAddWaitlistLeadRequest::SCHEMA,
            Self::ApiKeyConnect => ApiKeyConnectRequest::SCHEMA,
            Self::TokenIntrospect => TokenIntrospectRequest::SCHEMA,
            Self::SubscribeTokenRevocations => SubscribeTokenRevocationsRequest::SCHEMA,
        };
        serde_json::from_str(schema).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorXxx {}
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
    /// None Please populate error_codes.json
    Xxx = 0,
}

impl From<EnumErrorCode> for ErrorCode {
    fn from(e: EnumErrorCode) -> Self {
        ErrorCode::new(e as _)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddWaitlistLeadRequest {
    pub name: String,
    #[serde(default)]
    pub telegram: Option<String>,
    #[serde(default)]
    pub whatsApp: Option<String>,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddWaitlistLeadResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyConnectRequest {
    pub appPublicId: uuid::Uuid,
    pub appApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyConnectResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KardAddWaitlistLeadRequest {
    pub name: String,
    #[serde(default)]
    pub telegram: Option<String>,
    #[serde(default)]
    pub whatsApp: Option<String>,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KardAddWaitlistLeadResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicConnectRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicConnectResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenExchangeRequest {
    pub refreshToken: String,
    pub appPublicId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenExchangeResponse {
    pub accessToken: String,
    pub refreshToken: String,
    pub idToken: String,
    pub tokenType: String,
    pub expiresIn: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub agreedTos: String,
    pub agreedPrivacy: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupResponse {
    pub accessToken: String,
    pub refreshToken: String,
    pub idToken: String,
    pub tokenType: String,
    pub expiresIn: i32,
    pub profileCallbackUrl: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartAuthRequest {
    pub appPublicId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartAuthResponse {
    pub loginConfig: String,
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
    pub refreshToken: String,
    pub idToken: String,
    pub tokenType: String,
    pub expiresIn: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitUsernameRequest {
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitUsernameResponse {
    pub expiresAt: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeTokenRevocationsRequest {
    pub appPublicId: uuid::Uuid,
    pub appApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeTokenRevocationsResponse {
    pub subscriptionId: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenIntrospectRequest {
    pub token: String,
    #[serde(default)]
    pub tokenTypeHint: Option<String>,
    pub appPublicId: uuid::Uuid,
    pub appApiKey: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenIntrospectResponse {
    pub active: bool,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub appPublicId: Option<uuid::Uuid>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub userPublicId: Option<i64>,
    #[serde(default)]
    pub exp: Option<i64>,
    #[serde(default)]
    pub iat: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenRevokeRequest {
    pub token: String,
    #[serde(default)]
    pub tokenTypeHint: Option<String>,
    pub appPublicId: uuid::Uuid,
    #[serde(default)]
    pub clientSecret: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenRevokeResponse {
    pub success: bool,
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

impl WsRequest for StartAuthRequest {
    type Response = StartAuthResponse;
    const METHOD_ID: u32 = 10;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "StartAuth",
  "code": 10,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": "UUID"
    }
  ],
  "returns": [
    {
      "name": "loginConfig",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Frontend initiates HoneyAuth authorization flow. Session is stored per connection.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for StartAuthResponse {
    type Request = StartAuthRequest;
}

impl WsRequest for SignupRequest {
    type Response = SignupResponse;
    const METHOD_ID: u32 = 11;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "Signup",
  "code": 11,
  "parameters": [
    {
      "name": "username",
      "ty": "String"
    },
    {
      "name": "password",
      "ty": "String"
    },
    {
      "name": "agreedTos",
      "ty": "String"
    },
    {
      "name": "agreedPrivacy",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "accessToken",
      "ty": "String"
    },
    {
      "name": "refreshToken",
      "ty": "String"
    },
    {
      "name": "idToken",
      "ty": "String"
    },
    {
      "name": "tokenType",
      "ty": "String"
    },
    {
      "name": "expiresIn",
      "ty": "Int"
    },
    {
      "name": "profileCallbackUrl",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Frontend creates new user account during HoneyAuth signup. Session is per connection. Returns authorization code and profile callback endpoint.",
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
  "description": "Step 1: Frontend submits username during HoneyAuth flow. Session is per connection.",
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
    },
    {
      "name": "refreshToken",
      "ty": "String"
    },
    {
      "name": "idToken",
      "ty": "String"
    },
    {
      "name": "tokenType",
      "ty": "String"
    },
    {
      "name": "expiresIn",
      "ty": "Int"
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

impl WsRequest for RefreshTokenExchangeRequest {
    type Response = RefreshTokenExchangeResponse;
    const METHOD_ID: u32 = 14;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "RefreshTokenExchange",
  "code": 14,
  "parameters": [
    {
      "name": "refreshToken",
      "ty": "String"
    },
    {
      "name": "appPublicId",
      "ty": "UUID"
    }
  ],
  "returns": [
    {
      "name": "accessToken",
      "ty": "String"
    },
    {
      "name": "refreshToken",
      "ty": "String"
    },
    {
      "name": "idToken",
      "ty": "String"
    },
    {
      "name": "tokenType",
      "ty": "String"
    },
    {
      "name": "expiresIn",
      "ty": "Int"
    }
  ],
  "stream_response": null,
  "description": "Exchange refresh token for access tokens. Redirect URI handling is client-side; use WebSocket connection context for navigation.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for RefreshTokenExchangeResponse {
    type Request = RefreshTokenExchangeRequest;
}

impl WsRequest for TokenRevokeRequest {
    type Response = TokenRevokeResponse;
    const METHOD_ID: u32 = 15;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "TokenRevoke",
  "code": 15,
  "parameters": [
    {
      "name": "token",
      "ty": "String"
    },
    {
      "name": "tokenTypeHint",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "appPublicId",
      "ty": "UUID"
    },
    {
      "name": "clientSecret",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Revoke access or refresh tokens.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for TokenRevokeResponse {
    type Request = TokenRevokeRequest;
}

impl WsRequest for AddWaitlistLeadRequest {
    type Response = AddWaitlistLeadResponse;
    const METHOD_ID: u32 = 20;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "AddWaitlistLead",
  "code": 20,
  "parameters": [
    {
      "name": "name",
      "ty": "String"
    },
    {
      "name": "telegram",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "whatsApp",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Adds a lead to the waitlist.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for AddWaitlistLeadResponse {
    type Request = AddWaitlistLeadRequest;
}

impl WsRequest for KardAddWaitlistLeadRequest {
    type Response = KardAddWaitlistLeadResponse;
    const METHOD_ID: u32 = 30;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "KardAddWaitlistLead",
  "code": 30,
  "parameters": [
    {
      "name": "name",
      "ty": "String"
    },
    {
      "name": "telegram",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "whatsApp",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Adds a lead to the waitlist.",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for KardAddWaitlistLeadResponse {
    type Request = KardAddWaitlistLeadRequest;
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
      "name": "appPublicId",
      "ty": "UUID"
    },
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

impl WsRequest for TokenIntrospectRequest {
    type Response = TokenIntrospectResponse;
    const METHOD_ID: u32 = 110;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "TokenIntrospect",
  "code": 110,
  "parameters": [
    {
      "name": "token",
      "ty": "String"
    },
    {
      "name": "tokenTypeHint",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "appPublicId",
      "ty": "UUID"
    },
    {
      "name": "appApiKey",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "active",
      "ty": "Boolean"
    },
    {
      "name": "scope",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "appPublicId",
      "ty": {
        "Optional": "UUID"
      }
    },
    {
      "name": "username",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "userPublicId",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "exp",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "iat",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "stream_response": null,
  "description": "Backend validates HoneyAuth tokens (requires app API key).",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ]
}"#;
}
impl WsResponse for TokenIntrospectResponse {
    type Request = TokenIntrospectRequest;
}

impl WsRequest for SubscribeTokenRevocationsRequest {
    type Response = SubscribeTokenRevocationsResponse;
    const METHOD_ID: u32 = 111;
    const ROLES: &[u32] = &[6];
    const SCHEMA: &'static str = r#"{
  "name": "SubscribeTokenRevocations",
  "code": 111,
  "parameters": [
    {
      "name": "appPublicId",
      "ty": "UUID"
    },
    {
      "name": "appApiKey",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "subscriptionId",
      "ty": "String"
    }
  ],
  "stream_response": null,
  "description": "Backend subscribes to user token revocation events. TODO. This is not implemented",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ]
}"#;
}
impl WsResponse for SubscribeTokenRevocationsResponse {
    type Request = SubscribeTokenRevocationsRequest;
}
