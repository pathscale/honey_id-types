use std::sync::Arc;

use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::HoneyIdConfig;
use crate::tables::token::TokenWorkTable;
use crate::types::entity::UserPublicId;

mod connect;
mod submit_username;
mod token;

#[derive(Debug)]
pub struct HoneyIdClient {
    config: HoneyIdConfig,

    token_table: Arc<TokenWorkTable>,
}

impl HoneyIdClient {
    /// Created new [`HoneyIdClient`] with provided [`HoneyIdConfig`].
    #[must_use]
    pub fn new(config: HoneyIdConfig) -> Self {
        Self {
            config,
            token_table: Arc::new(Default::default()),
        }
    }

    pub fn get_app_pub_id(&self) -> Uuid {
        self.config.app_public_id
    }

    pub fn validate_auth_api_key(&self, key: &str) -> bool {
        self.config.auth_api_key.expose_secret() == key
    }
}
