//! Generic authentication handler that allows apps to define custom connection logic
//! without duplicating the common auth infrastructure.

use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use eyre::Result;
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing;
use uuid::Uuid;

use super::token_management::TokenStorage;
use super::user_management::UserStorage;
use crate::enums::HoneyErrorCode;
use crate::id_entities::UserPublicId;

/// Context passed to the `on_connect` callback for authorized connections.
/// Extensible: add fields here to give apps more info without changing their closure signature.
pub struct AuthorizedConnectContext {
    pub user_pub_id: UserPublicId,
    pub user_api_roles: Vec<u32>,
    pub conn: Arc<WsConnection>,
}

/// Context passed to the `on_connect` callback for public connections.
pub struct PublicConnectContext {
    pub conn: Arc<WsConnection>,
}

/// Trait app request types must implement to work with [`GenericAuthorizedConnect`].
/// The handler calls `get_access_token()` to extract the token for validation
/// without knowing the concrete request type.
pub trait AuthorizedConnectRequest: DeserializeOwned + Send + Sync + 'static {
    fn get_access_token(&self) -> &str;
}

/// Marker trait app request types must implement to work with [`GenericPublicConnect`].
/// No methods required — just constrains the type to be deserializable.
pub trait PublicConnectRequest: DeserializeOwned + Send + Sync + 'static {}

/// Trait for custom connection handling logic.
/// Implementations define app-specific behavior during WebSocket connection establishment.
#[async_trait(?Send)]
pub trait CustomConnectHandler: Send + Sync + 'static {
    /// Handle custom connection logic.
    /// The handler is responsible for:
    /// - Deserializing the request to the app's custom type
    /// - Performing any validation or authorization checks
    /// - Setting connection roles via `conn.set_roles(...)`
    /// - Returning the response as a JSON value
    ///
    /// # Arguments
    /// * `param` - The raw request payload as JSON
    /// * `conn` - The WebSocket connection to set up roles and metadata
    ///
    /// # Returns
    /// The response to send back to the client, as a JSON value
    async fn handle(&self, param: Value, conn: Arc<WsConnection>) -> Result<Value>;
}

/// Simple wrapper that implements `SubAuthController` to allow custom handlers to work with the WebSocket server.
/// This eliminates the need for each app to create a new struct implementing `SubAuthController`.
pub struct GenericConnectHandler {
    pub custom_handler: Arc<dyn CustomConnectHandler>,
}

#[async_trait(?Send)]
impl SubAuthController for GenericConnectHandler {
    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        param: Value,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Result<Value>> {
        let custom_handler = self.custom_handler.clone();
        async move { custom_handler.handle(param, conn).await }.boxed_local()
    }
}

/// Generic handler for authorized WebSocket connections with custom request/response types.
///
/// This handler validates access tokens, assigns roles, and delegates to a custom callback.
///
/// # Example
///
/// For custom types:
/// ```ignore
/// use crate::handlers::convenience_utils::generic_auth_handler::{
///     GenericAuthorizedConnect, AuthorizedConnectRequest, AuthorizedConnectContext,
/// };
///
/// struct MyRequest { access_token: String, extra: u32 }
/// struct MyResponse { message: String }
///
/// impl AuthorizedConnectRequest for MyRequest {
///     fn get_access_token(&self) -> &str { &self.access_token }
/// }
///
/// let handler = GenericAuthorizedConnect::new(token_storage, user_storage, |req, ctx| async move {
///     let user_id = ctx.user_pub_id;
///     let extra = req.extra;
///     Ok(MyResponse { message: format!("User {}", user_id) })
/// });
/// ```
///
/// For Honey types, see [`crate::handlers::user_to_app::MethodAuthorizedConnect`].
pub struct GenericAuthorizedConnect<Req, Res>
where
    Req: AuthorizedConnectRequest,
    Res: serde::Serialize + Send + Sync + 'static,
{
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
    pub user_storage: Arc<dyn UserStorage + Sync + Send>,
    on_connect: Arc<
        dyn Fn(Req, AuthorizedConnectContext) -> LocalBoxFuture<'static, Result<Res>> + Send + Sync,
    >,
    _phantom: PhantomData<(Req, Res)>,
}

impl<Req, Res> GenericAuthorizedConnect<Req, Res>
where
    Req: AuthorizedConnectRequest,
    Res: serde::Serialize + Send + Sync + 'static,
{
    /// Create a new authorized connect handler.
    ///
    /// # Arguments
    /// * `token_storage` - Storage for token validation
    /// * `user_storage` - Storage for user role lookup
    /// * `on_connect` - Async callback that receives the request and context
    pub fn new<F, Fut>(
        token_storage: Arc<dyn TokenStorage + Sync + Send>,
        user_storage: Arc<dyn UserStorage + Sync + Send>,
        on_connect: F,
    ) -> Self
    where
        F: Fn(Req, AuthorizedConnectContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Res>> + Send + 'static,
    {
        let on_connect_boxed = Arc::new(move |req, ctx| on_connect(req, ctx).boxed_local());
        Self {
            token_storage,
            user_storage,
            on_connect: on_connect_boxed,
            _phantom: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Req, Res> SubAuthController for GenericAuthorizedConnect<Req, Res>
where
    Req: AuthorizedConnectRequest,
    Res: serde::Serialize + Send + Sync + 'static,
{
    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        param: Value,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Result<Value>> {
        async move {
            let req: Req = serde_json::from_value(param).map_err(|x| {
                CustomError::new(HoneyErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            let token = Uuid::parse_str(req.get_access_token()).map_err(|x| {
                CustomError::new(HoneyErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            let Ok(user_pub_id) = self.token_storage.validate_token(token) else {
                tracing::error!(
                    error = "Wrong `accessToken`",
                    "`GenericAuthorizedConnect` failed to validate the `accessToken`."
                );
                return Err(
                    CustomError::new(HoneyErrorCode::BadRequest, "Wrong `accessToken`").into(),
                );
            };

            let roles = Vec::from(self.user_storage.get_api_roles_by_pub_id(user_pub_id)?);
            conn.set_roles(Arc::new(roles.clone()));

            let ctx = AuthorizedConnectContext {
                user_pub_id,
                user_api_roles: roles,
                conn,
            };
            let res = (self.on_connect)(req, ctx).await?;

            Ok(serde_json::to_value(res)?)
        }
        .boxed_local()
    }
}

/// Generic handler for public WebSocket connections with custom request/response types.
///
/// This handler assigns the public role and delegates to a custom callback.
///
/// # Example
///
/// For custom types:
/// ```ignore
/// use crate::handlers::convenience_utils::generic_auth_handler::{
///     GenericPublicConnect, PublicConnectRequest, PublicConnectContext,
/// };
///
/// struct MyPublicRequest { session_id: String }
/// struct MyPublicResponse { status: String }
///
/// impl PublicConnectRequest for MyPublicRequest {}
///
/// let handler = GenericPublicConnect::new(user_storage, |req, ctx| async move {
///     Ok(MyPublicResponse { status: "connected".to_string() })
/// });
/// ```
///
/// For Honey types, see [`crate::handlers::user_to_app::MethodPublicConnect`].
pub struct GenericPublicConnect<Req, Res>
where
    Req: PublicConnectRequest,
    Res: serde::Serialize + Send + Sync + 'static,
{
    pub user_storage: Arc<dyn UserStorage + Sync + Send>,
    on_connect: Arc<
        dyn Fn(Req, PublicConnectContext) -> LocalBoxFuture<'static, Result<Res>> + Send + Sync,
    >,
    _phantom: PhantomData<(Req, Res)>,
}

impl<Req, Res> GenericPublicConnect<Req, Res>
where
    Req: PublicConnectRequest,
    Res: serde::Serialize + Send + Sync + 'static,
{
    /// Create a new public connect handler.
    ///
    /// # Arguments
    /// * `user_storage` - Storage for role lookup
    /// * `on_connect` - Async callback that receives the request and context
    pub fn new<F, Fut>(user_storage: Arc<dyn UserStorage + Sync + Send>, on_connect: F) -> Self
    where
        F: Fn(Req, PublicConnectContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Res>> + Send + 'static,
    {
        let on_connect_boxed = Arc::new(move |req, ctx| on_connect(req, ctx).boxed_local());
        Self {
            user_storage,
            on_connect: on_connect_boxed,
            _phantom: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Req, Res> SubAuthController for GenericPublicConnect<Req, Res>
where
    Req: PublicConnectRequest,
    Res: serde::Serialize + Send + Sync + 'static,
{
    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        param: Value,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Result<Value>> {
        async move {
            let req: Req = serde_json::from_value(param).map_err(|x| {
                CustomError::new(HoneyErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            let roles = self.user_storage.get_public_roles();
            conn.set_roles(Arc::new(Vec::from(roles)));

            let ctx = PublicConnectContext { conn };
            let res = (self.on_connect)(req, ctx).await?;

            Ok(serde_json::to_value(res)?)
        }
        .boxed_local()
    }
}

/// Wrapper that allows closures to be used as `CustomConnectHandler` implementations.
/// This enables inline handler definitions without creating separate types.
///
/// # Example
/// ```ignore
/// let handler = ClosureHandler(|param, conn| async move {
///     // Custom logic here
///     Ok(Value::Null)
/// });
/// ```
pub struct ClosureHandler<F>(pub F);

#[async_trait(?Send)]
impl<F> CustomConnectHandler for ClosureHandler<F>
where
    F: Fn(Value, Arc<WsConnection>) -> LocalBoxFuture<'static, Result<Value>>
        + Send
        + Sync
        + 'static,
{
    async fn handle(&self, param: Value, conn: Arc<WsConnection>) -> Result<Value> {
        (self.0)(param, conn).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_handler_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<
            ClosureHandler<fn(Value, Arc<WsConnection>) -> LocalBoxFuture<'static, Result<Value>>>,
        >();
    }
}
