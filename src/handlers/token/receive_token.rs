use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::handler::{RequestHandler, Response};
use endpoint_libs::libs::toolbox::RequestContext;

use crate::endpoints::callback::{ReceiveTokenRequest, ReceiveTokenResponse};
use crate::types::traits::TokenStorage;

pub struct MethodReceiveToken {
    pub token_storage: Arc<dyn TokenStorage + Sync + Send>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveToken {
    type Request = ReceiveTokenRequest;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request> {
        let token = uuid::Uuid::parse_str(&req.token)?;
        let user_pub_id = crate::types::entity::UserPublicId::from(req.userPubId);

        self.token_storage.store_token(user_pub_id, token)?;

        Ok(ReceiveTokenResponse {})
    }
}
