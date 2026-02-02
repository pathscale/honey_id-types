#![allow(unused_imports)]

pub mod connect {
    pub use crate::types::generated::AccessTokenConnectRequest as HoneyAccessTokenConnectRequest;
    pub use crate::types::generated::AccessTokenConnectResponse as HoneyAccessTokenConnectResponse;
    pub use crate::types::generated::ApiKeyConnectRequest as HoneyApiKeyConnectRequest;
    pub use crate::types::generated::ApiKeyConnectResponse as HoneyApiKeyConnectResponse;
    pub use crate::types::generated::PublicConnectRequest as HoneyPublicConnectRequest;
    pub use crate::types::generated::PublicConnectResponse as HoneyPublicConnectResponse;
}

pub mod callback {
    pub use crate::types::generated::{
        ReceiveTokenRequest as HoneyReceiveTokenRequest,
        ReceiveTokenResponse as HoneyReceiveTokenResponse,
    };
    pub use crate::types::generated::{
        ReceiveUserInfoRequest as HoneyReceiveUserInfoRequest,
        ReceiveUserInfoResponse as HoneyReceiveUserInfoResponse,
    };
}

pub mod auth_flow {
    pub use crate::types::generated::SignupRequest as HoneySignupRequest;
    pub use crate::types::generated::SignupResponse as HoneySignupResponse;
    pub use crate::types::generated::SubmitPasswordRequest as HoneySubmitPasswordRequest;
    pub use crate::types::generated::SubmitPasswordResponse as HoneySubmitPasswordResponse;
    pub use crate::types::generated::SubmitUsernameRequest as HoneySubmitUsernameRequest;
    pub use crate::types::generated::SubmitUsernameResponse as HoneySubmitUsernameResponse;
}

pub mod platform {
    pub use crate::types::generated::{BanUserRequest, BanUserResponse};
    pub use crate::types::generated::{CreateAppConfigRequest, CreateAppConfigResponse};
    pub use crate::types::generated::{DeleteAppConfigRequest, DeleteAppConfigResponse};
    pub use crate::types::generated::{DeleteUserRequest, DeleteUserResponse};
    pub use crate::types::generated::{EditAppConfigRequest, EditAppConfigResponse};
    pub use crate::types::generated::{EditUserRequest, EditUserResponse};
    pub use crate::types::generated::{GetAppSecurityRulesRequest, GetAppSecurityRulesResponse};
    pub use crate::types::generated::{PlatformConnectRequest, PlatformConnectResponse};

    pub mod endpoint_codes {
        pub use crate::types::generated::EnumEndpoint::BanUser;
        pub use crate::types::generated::EnumEndpoint::CreateAppConfig;
        pub use crate::types::generated::EnumEndpoint::DeleteAppConfig;
        pub use crate::types::generated::EnumEndpoint::DeleteUser;
        pub use crate::types::generated::EnumEndpoint::EditAppConfig;
        pub use crate::types::generated::EnumEndpoint::EditUser;
        pub use crate::types::generated::EnumEndpoint::GetAppSecurityRules;
        pub use crate::types::generated::EnumEndpoint::PlatformConnect;
    }
}
