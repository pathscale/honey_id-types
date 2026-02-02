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
