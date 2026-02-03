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
    /// The role is used for platform only.
    Platform = 7,
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
pub enum EnumUserStatus {
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
    EditUser = 113,
    ///
    DeleteUser = 114,
    ///
    DeleteAppConfig = 115,
    ///
    EditAppConfig = 116,
    ///
    GetAppSecurityRules = 117,
    ///
    ApiKeyConnect = 200,
    ///
    AccessTokenConnect = 201,
    ///
    ReceiveToken = 210,
    ///
    ReceiveUserInfo = 211,
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
            Self::EditUser => EditUserRequest::SCHEMA,
            Self::DeleteUser => DeleteUserRequest::SCHEMA,
            Self::DeleteAppConfig => DeleteAppConfigRequest::SCHEMA,
            Self::EditAppConfig => EditAppConfigRequest::SCHEMA,
            Self::GetAppSecurityRules => GetAppSecurityRulesRequest::SCHEMA,
            Self::ApiKeyConnect => ApiKeyConnectRequest::SCHEMA,
            Self::AccessTokenConnect => AccessTokenConnectRequest::SCHEMA,
            Self::ReceiveToken => ReceiveTokenRequest::SCHEMA,
            Self::ReceiveUserInfo => ReceiveUserInfoRequest::SCHEMA,
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
pub struct AccessTokenConnectRequest {
    pub accessToken: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenConnectResponse {}
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
pub struct BanUserRequest {
    pub userPublicId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BanUserResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppConfigRequest {
    pub appPublicId: uuid::Uuid,
    pub callBackUrl: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppConfigResponse {
    pub appPublicId: uuid::Uuid,
    pub createdAt: i64,
    pub appApiKey: String,
    pub minPasswordLength: i32,
    pub requiredPasswordChars: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAppConfigRequest {
    pub appPublicId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAppConfigResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    pub appPublicId: uuid::Uuid,
    pub userPublicId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditAppConfigRequest {
    pub appPublicId: uuid::Uuid,
    #[serde(default)]
    pub minPasswordLength: Option<i32>,
    #[serde(default)]
    pub requiredPasswordChars: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditAppConfigResponse {
    pub appPublicId: uuid::Uuid,
    pub minPasswordLength: i32,
    pub requiredPasswordChars: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditUserRequest {
    pub userPublicId: uuid::Uuid,
    pub newStatus: EnumUserStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditUserResponse {
    pub userPublicId: uuid::Uuid,
    pub newStatus: EnumUserStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAppSecurityRulesRequest {
    pub appPublicId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAppSecurityRulesResponse {
    pub appPublicId: uuid::Uuid,
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
    pub userPubId: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveTokenResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveUserInfoRequest {
    pub userPubId: uuid::Uuid,
    pub username: String,
    pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveUserInfoResponse {}
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
      "ty": "UUID"
    },
    {
      "name": "callBackUrl",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": "UUID"
    },
    {
      "name": "createdAt",
      "ty": "BigInt"
    },
    {
      "name": "appApiKey",
      "ty": "String"
    },
    {
      "name": "minPasswordLength",
      "ty": "Int"
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
      "ty": "UUID"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Ban a user",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ]
}"#;
}
impl WsResponse for BanUserResponse {
    type Request = BanUserRequest;
}

impl WsRequest for EditUserRequest {
    type Response = EditUserResponse;
    const METHOD_ID: u32 = 113;
    const ROLES: &[u32] = &[7];
    const SCHEMA: &'static str = r#"{
  "name": "EditUser",
  "code": 113,
  "parameters": [
    {
      "name": "userPublicId",
      "ty": "UUID"
    },
    {
      "name": "newStatus",
      "ty": {
        "EnumRef": "UserStatus"
      }
    }
  ],
  "returns": [
    {
      "name": "userPublicId",
      "ty": "UUID"
    },
    {
      "name": "newStatus",
      "ty": {
        "EnumRef": "UserStatus"
      }
    }
  ],
  "stream_response": null,
  "description": "Edit user status",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
  ]
}"#;
}
impl WsResponse for EditUserResponse {
    type Request = EditUserRequest;
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
      "ty": "UUID"
    },
    {
      "name": "userPublicId",
      "ty": "UUID"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Delete a user",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
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
      "ty": "UUID"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Delete app configuration",
  "json_schema": null,
  "roles": [
    "UserRole::Platform"
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
      "ty": "UUID"
    },
    {
      "name": "minPasswordLength",
      "ty": {
        "Optional": "Int"
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
      "ty": "UUID"
    },
    {
      "name": "minPasswordLength",
      "ty": "Int"
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
      "ty": "UUID"
    }
  ],
  "returns": [
    {
      "name": "appPublicId",
      "ty": "UUID"
    },
    {
      "name": "minPasswordLength",
      "ty": "Int"
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
  ]
}"#;
}
impl WsResponse for GetAppSecurityRulesResponse {
    type Request = GetAppSecurityRulesRequest;
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
  ]
}"#;
}
impl WsResponse for ApiKeyConnectResponse {
    type Request = ApiKeyConnectRequest;
}

impl WsRequest for AccessTokenConnectRequest {
    type Response = AccessTokenConnectResponse;
    const METHOD_ID: u32 = 201;
    const ROLES: &[u32] = &[0];
    const SCHEMA: &'static str = r#"{
  "name": "AccessTokenConnect",
  "code": 201,
  "parameters": [
    {
      "name": "accessToken",
      "ty": "UUID"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null,
  "roles": [
    "UserRole::Public"
  ]
}"#;
}
impl WsResponse for AccessTokenConnectResponse {
    type Request = AccessTokenConnectRequest;
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
      "ty": "UUID"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Backend receives auth tokens, happens after login or signup",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
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
      "ty": "UUID"
    },
    {
      "name": "username",
      "ty": "String"
    },
    {
      "name": "token",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "Backend receives user info with token, happens after new user signs up.",
  "json_schema": null,
  "roles": [
    "UserRole::AppApiKey"
  ]
}"#;
}
impl WsResponse for ReceiveUserInfoResponse {
    type Request = ReceiveUserInfoRequest;
}
