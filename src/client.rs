use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::HoneyIdConfig;

mod connect;
mod submit_username;

#[derive(Debug)]
pub struct HoneyIdClient {
    config: HoneyIdConfig,
}

impl HoneyIdClient {
    /// Created new [`HoneyIdClient`] with provided [`HoneyIdConfig`].
    #[must_use]
    pub fn new(config: HoneyIdConfig) -> Self {
        Self {
            config,
        }
    }

    pub fn get_app_pub_id(&self) -> Uuid {
        self.config.app_public_id
    }

    pub fn validate_auth_api_key(&self, key: &str) -> bool {
        self.config.auth_api_key.expose_secret() == key
    }
}
