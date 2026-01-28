mod start_auth;
mod connect;

use uuid::Uuid;

use crate::HoneyIdConfig;

#[derive(Debug)]
pub struct HoneyIdClient {
    config: HoneyIdConfig
}

impl HoneyIdClient {
    /// Created new [`HoneyIdClient`] with provided [`HoneyIdConfig`].
    #[must_use]
    pub fn new(config: HoneyIdConfig) -> Self {
        Self { config }
    }

    pub fn get_app_pub_id(&self) -> Uuid {
        self.config.app_public_id
    }
}