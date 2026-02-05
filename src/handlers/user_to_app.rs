//! Defines all handlers that are for User (FE) to App (BE) communication
use crate::endpoints::connect::{
    HoneyAuthorizedConnectRequest, HoneyAuthorizedConnectResponse, HoneyPublicConnectRequest,
    HoneyPublicConnectResponse,
};
use crate::handlers::convenience_utils::generic_auth_handler::{
    AuthorizedConnectRequest, GenericAuthorizedConnect, GenericPublicConnect, PublicConnectRequest,
};

// Trait implementations for Honey types
impl AuthorizedConnectRequest for HoneyAuthorizedConnectRequest {
    fn get_access_token(&self) -> &str {
        &self.accessToken
    }
}

impl PublicConnectRequest for HoneyPublicConnectRequest {}

// Type aliases for convenience
/// Type alias for authorized WebSocket connections using Honey types.
///
/// # Usage
///
/// Downstream code creates instances via:
/// ```ignore
/// MethodAuthorizedConnect::new(token_storage, user_storage, |_req, _ctx| async {
///     Ok(HoneyAuthorizedConnectResponse {})
/// })
/// ```
pub type MethodAuthorizedConnect =
    GenericAuthorizedConnect<HoneyAuthorizedConnectRequest, HoneyAuthorizedConnectResponse>;

/// Type alias for public WebSocket connections using Honey types.
///
/// # Usage
///
/// Downstream code creates instances via:
/// ```ignore
/// MethodPublicConnect::new(user_storage, |_req, _ctx| async {
///     Ok(HoneyPublicConnectResponse {})
/// })
/// ```
pub type MethodPublicConnect =
    GenericPublicConnect<HoneyPublicConnectRequest, HoneyPublicConnectResponse>;
