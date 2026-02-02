use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use eyre::{Result, bail};
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use serde_json::Value;

use crate::endpoints::connect::{AccessTokenConnectRequest, AccessTokenConnectResponse};
use crate::enums::ErrorCode;
use crate::types::traits::{TokenStorage, UserStorage};

pub struct MethodAccessTokenConnect {
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
    pub user_storage: Arc<dyn UserStorage + Sync + Send>,
}

#[async_trait(?Send)]
impl SubAuthController for MethodAccessTokenConnect {
    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        param: Value,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Result<Value>> {
        async move {
            let req: AccessTokenConnectRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(ErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            let Ok(user_pub_id) = self.token_storage.validate_token(req.accessToken) else {
                tracing::error!(
                    error = "Wrong `accessToken`",
                    "`AccessTokenConnect` failed to validate the `accessToken`."
                );
                bail!(CustomError::new(
                    ErrorCode::BadRequest,
                    "Wrong `accessToken`"
                ))
            };

            let role = self.user_storage.get_role_by_pub_id(user_pub_id)?;

            conn.set_roles(Arc::new(vec![role]));

            Ok(serde_json::to_value(AccessTokenConnectResponse {})?)
        }
        .boxed_local()
    }
}
