//! Defines all handlers that are for Auth (BE) to App (BE) communication
use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::handler::{HandlerError, RequestHandler, Response};
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use uuid::Uuid;

use crate::client::{ApiKeyError, HoneyIdClient};
use crate::endpoints::callback::{
    HoneyReceiveTokenError, HoneyReceiveTokenRequest, HoneyReceiveTokenResponse, HoneyReceiveUserDeletedRequest,
    HoneyReceiveUserDeletedResponse, HoneyReceiveUserInfoError, HoneyReceiveUserInfoRequest,
    HoneyReceiveUserInfoResponse, HoneyValidateTokenError, HoneyValidateTokenRequest, HoneyValidateTokenResponse,
};
use crate::endpoints::connect::{HoneyApiKeyConnectError, HoneyApiKeyConnectRequest, HoneyApiKeyConnectResponse};
use crate::handlers::convenience_utils::token_management::TokenStorage;
use crate::handlers::convenience_utils::user_management::{CreateUserInfo, DeleteUserInfo, UserStorage};
use crate::types::id_entities::UserPublicId;

pub struct MethodApiKeyConnect {
    pub honey_id_client: Arc<HoneyIdClient>,
    pub user_storage: Arc<dyn UserStorage + Send + Sync>,
}

#[async_trait(?Send)]
impl SubAuthController for MethodApiKeyConnect {
    type Request = HoneyApiKeyConnectRequest;
    type Error = HoneyApiKeyConnectError;

    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        req: Self::Request,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Response<Self::Request, Self::Error>> {
        async move {
            self.honey_id_client
                .validate_auth_api_key(&req.appApiKey)
                .map_err(|err| {
                    tracing::error!(
                        error = %err,
                        "Failed to validate Auth API key due to error"
                    );
                    match err {
                        ApiKeyError::IncorrectKey => HandlerError::Public(HoneyApiKeyConnectError::InvalidApiKey),
                    }
                })?;

            let auth_role = self.user_storage.get_honey_auth_role();
            conn.set_roles(Arc::new(vec![auth_role]));

            Ok(HoneyApiKeyConnectResponse {})
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
    type Error = HoneyReceiveTokenError;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request, Self::Error> {
        let token = uuid::Uuid::parse_str(&req.token)
            .map_err(|_| HandlerError::Public(HoneyReceiveTokenError::InvalidToken))?;
        let user_pub_id = UserPublicId::from(req.userPubId);

        self.user_storage
            .create_or_update_user(CreateUserInfo {
                username: req.username,
                user_pub_id: req.userPubId,
                app_pub_id: None,
            })
            .await
            .map_err(HandlerError::internal)?;

        self.token_storage
            .store_token(user_pub_id, token)
            .await
            .map_err(HandlerError::internal)?;

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
    type Error = HoneyReceiveUserInfoError;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request, Self::Error> {
        let user_pub_id = UserPublicId::from(req.userPubId);

        self.user_storage
            .create_or_update_user(CreateUserInfo {
                username: req.username,
                user_pub_id: req.userPubId,
                app_pub_id: req.appPubId,
            })
            .await
            .map_err(HandlerError::internal)?;

        if let Some(token) = req.token {
            self.token_storage
                .store_token(
                    user_pub_id,
                    Uuid::try_parse(&token)
                        .map_err(|_| HandlerError::Public(HoneyReceiveUserInfoError::InvalidToken))?,
                )
                .await
                .map_err(HandlerError::internal)?;
        }

        Ok(HoneyReceiveUserInfoResponse {})
    }
}

pub struct MethodReceiveUserDeleted {
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
    pub user_storage: Arc<dyn UserStorage + Send + Sync>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveUserDeleted {
    type Request = HoneyReceiveUserDeletedRequest;
    type Error = CustomError;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request, Self::Error> {
        let user_pub_id = UserPublicId::from(req.userPubId);

        self.token_storage
            .remove_tokens_for_user(user_pub_id)
            .await
            .map_err(HandlerError::internal)?;

        self.user_storage
            .delete_user(DeleteUserInfo {
                user_pub_id: req.userPubId,
                app_pub_id: req.appPubId,
            })
            .await
            .map_err(HandlerError::internal)?;

        Ok(HoneyReceiveUserDeletedResponse {})
    }
}

pub struct MethodValidateToken {
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodValidateToken {
    type Request = HoneyValidateTokenRequest;
    type Error = HoneyValidateTokenError;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request, Self::Error> {
        let token =
            Uuid::parse_str(&req.token).map_err(|_| HandlerError::Public(HoneyValidateTokenError::InvalidToken))?;

        match self.token_storage.validate_token(token).await {
            Ok(user_pub_id) => Ok(HoneyValidateTokenResponse {
                valid: true,
                userPubId: Some(user_pub_id.into()),
            }),
            Err(_) => Ok(HoneyValidateTokenResponse {
                valid: false,
                userPubId: None,
            }),
        }
    }
}
