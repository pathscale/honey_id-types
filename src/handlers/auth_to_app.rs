//! Defines all handlers that are for Auth (BE) to App (BE) communication
use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::handler::{RequestHandler, Response};
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use eyre::{Context, Result, bail};
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use serde_json::Value;
use uuid::Uuid;

use crate::client::HoneyIdClient;
use crate::endpoints::callback::{
    HoneyReceiveTokenRequest, HoneyReceiveTokenResponse, HoneyReceiveUserInfoRequest,
    HoneyReceiveUserInfoResponse,
};
use crate::endpoints::connect::{HoneyApiKeyConnectRequest, HoneyApiKeyConnectResponse};
use crate::enums::HoneyErrorCode;
use crate::handlers::convenience_utils::token_management::TokenStorage;
use crate::handlers::convenience_utils::user_management::{CreateUserInfo, UserStorage};

pub struct MethodApiKeyConnect {
    pub honey_id_client: Arc<HoneyIdClient>,
    pub user_storage: Arc<dyn UserStorage + Send + Sync>,
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
            let req: HoneyApiKeyConnectRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(HoneyErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            if let Some(valid) = self.honey_id_client.validate_auth_api_key(&req.appApiKey) {
                if !valid {
                    tracing::error!(
                        error = "Wrong `authApiKey`",
                        "`ApiKeyConnect` failed to validate the auth api key."
                    );
                    // TODO: Define actual error codes specific to Honey
                    bail!(CustomError::new(
                        HoneyErrorCode::BadRequest,
                        "Wrong `authApiKey`"
                    ))
                }
            } else {
                bail!(CustomError::new(
                        HoneyErrorCode::InternalError,
                        "authApiKey has not been configured within Honey API app. App needs to be configured and restarted"
                    ))
            }

            let auth_role = self.user_storage.get_honey_auth_role();
            conn.set_roles(Arc::new(vec![auth_role]));

            Ok(serde_json::to_value(HoneyApiKeyConnectResponse {})?)
        }
        .boxed_local()
    }
}

pub struct MethodReceiveToken {
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
    pub user_storage: Arc<dyn UserStorage + Send + Sync>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveToken {
    type Request = HoneyReceiveTokenRequest;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request> {
        let token = uuid::Uuid::parse_str(&req.token)?;
        let user_pub_id = crate::types::id_entities::UserPublicId::from(req.userPubId);

        self.user_storage
            .create_or_update_user(CreateUserInfo {
                username: req.username,
                user_pub_id: req.userPubId,
                app_pub_id: None,
            })
            .await?;

        self.token_storage.store_token(user_pub_id, token)?;

        Ok(HoneyReceiveTokenResponse {})
    }
}
pub struct MethodReceiveUserInfo {
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
    pub user_storage: Arc<dyn UserStorage + Send + Sync>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveUserInfo {
    type Request = HoneyReceiveUserInfoRequest;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request> {
        let user_pub_id = crate::types::id_entities::UserPublicId::from(req.userPubId);

        self.user_storage
            .create_or_update_user(CreateUserInfo {
                username: req.username,
                user_pub_id: req.userPubId,
                app_pub_id: req.appPubId,
            })
            .await?;

        if let Some(token) = req.token {
            self.token_storage.store_token(
                user_pub_id,
                Uuid::try_parse(&token).wrap_err("Error parsing given token as UUID")?,
            )?;
        }

        Ok(HoneyReceiveUserInfoResponse {})
    }
}
