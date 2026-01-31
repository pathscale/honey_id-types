use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::handler::{RequestHandler, Response};
use endpoint_libs::libs::toolbox::RequestContext;

use crate::api_key_endpoints::callback::{ReceiveTokenRequest, ReceiveTokenResponse};
use crate::client::HoneyIdClient;

pub struct MethodReceiveToken {
    pub honey_id_client: Arc<HoneyIdClient>,
}

#[async_trait(?Send)]
impl RequestHandler for MethodReceiveToken {
    type Request = ReceiveTokenRequest;

    async fn handle(&self, _ctx: RequestContext, req: Self::Request) -> Response<Self::Request> {
        let token = uuid::Uuid::parse_str(&req.token)?;
        let user_pub_id = crate::types::entity::UserPublicId::from(req.userPubId);

        self.honey_id_client.store_token(token, user_pub_id)?;

        Ok(ReceiveTokenResponse {})
    }
}
