use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use eyre::{Result, bail};
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use serde_json::Value;

use crate::endpoints::connect::{ApiKeyConnectRequest, ApiKeyConnectResponse};
use crate::client::HoneyIdClient;
use crate::enums::ErrorCode;

pub struct MethodApiKeyConnect {
    pub honey_id_client: Arc<HoneyIdClient>,
}

#[async_trait(?Send)]
impl SubAuthController for MethodApiKeyConnect {
    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        param: Value,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Result<Value>> {
        async move {
            let req: ApiKeyConnectRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(ErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            if !self.honey_id_client.validate_auth_api_key(&req.appApiKey) {
                tracing::error!(
                    error = "Wrong `authApiKey`",
                    "`ApiKeyConnect` failed to validate the auth api key."
                );
                bail!(CustomError::new(
                    ErrorCode::BadRequest,
                    "Wrong `authApiKey`"
                ))
            }

            conn.set_roles(Arc::new(vec![u32::MAX]));

            Ok(serde_json::to_value(ApiKeyConnectResponse {})?)
        }
        .boxed_local()
    }
}
