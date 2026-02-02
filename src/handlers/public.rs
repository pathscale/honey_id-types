use std::sync::Arc;

use async_trait::async_trait;
use endpoint_libs::libs::toolbox::{ArcToolbox, CustomError, RequestContext};
use endpoint_libs::libs::ws::{SubAuthController, WsConnection};
use eyre::{Result, bail};
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use serde_json::Value;

use crate::endpoints::connect::{
    AccessTokenConnectRequest, AccessTokenConnectResponse, PublicConnectRequest,
    PublicConnectResponse,
};
use crate::enums::ErrorCode;
use crate::types::traits::{TokenStorage, UserStorage};

pub struct MethodPublicConnect {
    pub user_storage: Arc<dyn UserStorage + Sync + Send>,
}

#[async_trait(?Send)]
impl SubAuthController for MethodPublicConnect {
    fn auth(
        self: Arc<Self>,
        _toolbox: &ArcToolbox,
        param: Value,
        _ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> LocalBoxFuture<'static, Result<Value>> {
        async move {
            let _req: PublicConnectRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(ErrorCode::BadRequest, format!("Invalid request: {x}"))
            })?;

            let role = self.user_storage.get_public_role();

            conn.set_roles(Arc::new(vec![role]));

            Ok(serde_json::to_value(PublicConnectResponse {})?)
        }
        .boxed_local()
    }
}
