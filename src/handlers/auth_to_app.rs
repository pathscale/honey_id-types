//! Defines all handlers that are for Auth (BE) to App (BE) communication
use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::handler::{HandlerError, RequestHandler, Response};
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use futures::FutureExt;
use futures::future::LocalBoxFuture;

use crate::client::{ApiKeyError, HoneyIdClient};
use crate::endpoints::callback::{
    HoneyReceiveTokenError, HoneyReceiveTokenRequest, HoneyReceiveTokenResponse, HoneyReceiveUserDeletedRequest,
    HoneyReceiveUserDeletedResponse, HoneyReceiveUserInfoError, HoneyReceiveUserInfoRequest,
    HoneyReceiveUserInfoResponse, HoneyValidateTokenError, HoneyValidateTokenRequest, HoneyValidateTokenResponse,
};
use crate::endpoints::connect::{HoneyApiKeyConnectError, HoneyApiKeyConnectRequest, HoneyApiKeyConnectResponse};
use crate::handlers::convenience_utils::completion_receipt::{
    CompletionId, CompletionReceiptError, ReceiveTokenCompletionKey, ReceiveTokenCompletionPayload,
    ReceiveTokenCompletionStorage,
};
use crate::handlers::convenience_utils::token_management::TokenStorage;
use crate::handlers::convenience_utils::user_management::{CreateUserInfo, DeleteUserInfo, UserStorage};
use crate::id_entities::{AppPublicId, AuthToken};
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
        if req.completionId.is_some() {
            return Err(HandlerError::Public(HoneyReceiveTokenError::CompletionNotSupported));
        }

        let token = req
            .token
            .parse::<AuthToken>()
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

        Ok(HoneyReceiveTokenResponse { completionId: None })
    }
}

/// Completion-aware ReceiveToken handler. Requests without a completion ID
/// retain the legacy callback behavior. Requests with an ID require a
/// revocation-aware adapter and receive an echoed ID only after the adapter
/// completes or recognizes a completed receipt. Register
/// `MethodReceiveUserDeletedWithCompletion` with this same adapter before
/// enabling this handler, and route every deletion/revocation path for these
/// users through that adapter instead of the legacy token/user delete handler.
pub struct MethodReceiveTokenWithCompletion {
    /// Use the same configured client Arc as `MethodApiKeyConnect`. Its
    /// configuration binds the accepted callback API key and receiver app ID.
    /// A receiver instance serves one app; a multi-app receiver must carry the
    /// authenticated app identity through its auth context instead.
    pub honey_id_client: Arc<HoneyIdClient>,
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
    pub user_storage: Arc<dyn UserStorage + Send + Sync>,
    pub completion_storage: Arc<dyn ReceiveTokenCompletionStorage>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveTokenWithCompletion {
    type Request = HoneyReceiveTokenRequest;
    type Error = HoneyReceiveTokenError;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request, Self::Error> {
        let token = req
            .token
            .parse::<AuthToken>()
            .map_err(|_| HandlerError::Public(HoneyReceiveTokenError::InvalidToken))?;
        let completion_id = req
            .completionId
            .as_deref()
            .map(CompletionId::parse)
            .transpose()
            .map_err(|_| HandlerError::Public(HoneyReceiveTokenError::InvalidCompletionId))?;
        let user_pub_id = UserPublicId::from(req.userPubId);
        let payload = ReceiveTokenCompletionPayload {
            token,
            username: req.username,
            user_pub_id,
        };

        if let Some(completion_id) = completion_id.as_ref() {
            let fingerprint_material = payload.fingerprint_material();
            self.completion_storage
                .apply_receive_token(
                    ReceiveTokenCompletionKey {
                        app_pub_id: AppPublicId::from(self.honey_id_client.get_app_pub_id()),
                        completion_id: completion_id.clone(),
                    },
                    &fingerprint_material,
                    &payload,
                )
                .await
                .map_err(map_completion_error)?;

            return Ok(HoneyReceiveTokenResponse {
                completionId: Some(completion_id.as_str().to_owned()),
            });
        }

        self.user_storage
            .create_or_update_user(CreateUserInfo {
                username: payload.username.clone(),
                user_pub_id: req.userPubId,
                app_pub_id: None,
            })
            .await
            .map_err(HandlerError::internal)?;

        self.token_storage
            .store_token(payload.user_pub_id, payload.token)
            .await
            .map_err(HandlerError::internal)?;

        Ok(HoneyReceiveTokenResponse { completionId: None })
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
                    token
                        .parse::<AuthToken>()
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

/// Revocation-aware delete route paired with `MethodReceiveTokenWithCompletion`.
/// Both handlers must share the same adapter instance so deletion installs the
/// tombstone checked by pending/completed receipt processing. Do not also
/// register the legacy delete route for these users.
pub struct MethodReceiveUserDeletedWithCompletion {
    /// Use the same configured client Arc as `MethodApiKeyConnect` and
    /// `MethodReceiveTokenWithCompletion` so the request scope is tied to the
    /// app whose API key authenticated this receiver.
    pub honey_id_client: Arc<HoneyIdClient>,
    pub completion_storage: Arc<dyn ReceiveTokenCompletionStorage>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveUserDeletedWithCompletion {
    type Request = HoneyReceiveUserDeletedRequest;
    type Error = CustomError;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request, Self::Error> {
        let configured_app_pub_id = AppPublicId::from(self.honey_id_client.get_app_pub_id());
        let app_pub_id = match req.appPubId {
            Some(request_app_pub_id) => {
                let request_app_pub_id = AppPublicId::from(request_app_pub_id);
                if request_app_pub_id != configured_app_pub_id {
                    return Err(HandlerError::internal(eyre::eyre!(
                        "ReceiveUserDeleted appPubId does not match the API key bound to this receiver"
                    )));
                }
                Some(request_app_pub_id)
            }
            None => None,
        };

        self.completion_storage
            .revoke_user(app_pub_id, UserPublicId::from(req.userPubId))
            .await
            .map_err(HandlerError::internal)?;

        Ok(HoneyReceiveUserDeletedResponse {})
    }
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
        let token = req
            .token
            .parse::<AuthToken>()
            .map_err(|_| HandlerError::Public(HoneyValidateTokenError::InvalidToken))?;

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

fn map_completion_error(error: CompletionReceiptError) -> HandlerError<HoneyReceiveTokenError> {
    match error {
        CompletionReceiptError::Conflict => HandlerError::Public(HoneyReceiveTokenError::CompletionConflict),
        CompletionReceiptError::Revoked => HandlerError::Public(HoneyReceiveTokenError::CompletionRevoked),
        unsupported @ (CompletionReceiptError::Unsupported | CompletionReceiptError::Storage(_)) => {
            HandlerError::internal(unsupported)
        }
    }
}
