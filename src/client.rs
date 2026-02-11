use secrecy::ExposeSecret;
use url::Url;
use uuid::Uuid;

use crate::{
    HoneyIdConfig,
    client::honeyid_ws_conn::HoneyIdConnection,
    endpoints::auth_flow::{
        HoneySubmitPasswordRequest, HoneySubmitPasswordResponse, HoneySubmitUsernameRequest,
        HoneySubmitUsernameResponse,
    },
    enums::HoneyEndpointMethodCode,
    types::error::HoneyIdResult,
};

pub mod honeyid_ws_conn;

#[derive(Debug)]
pub struct HoneyIdClient {
    config: HoneyIdConfig,
}

#[derive(Debug, Clone, derive_more::Display)]
pub enum ApiKeyError {
    #[display("Auth API key not configured in App. Please configure and restart the app")]
    NotConfigured,
    #[display(
        "Incorrect Auth API key provided to App. Either reconfigure with the correct key on the App's server, or report an Auth server bug"
    )]
    IncorrectKey,
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

    pub fn validate_auth_api_key(&self, key: &str) -> Result<(), ApiKeyError> {
        match &self.config.auth_api_key {
            Some(config_key) => {
                if config_key.expose_secret() == key {
                    Ok(())
                } else {
                    Err(ApiKeyError::IncorrectKey)
                }
            }
            None => Err(ApiKeyError::NotConfigured),
        }
    }

    // TODOVEON: signup call for use in API

    pub async fn sign_in(&self, username: &str, password: &str) -> HoneyIdResult<String> {
        let (_username_response, mut session_conn) = self.submit_username(username).await?;

        // TODO: Check expires at timestamp, not really necessary though
        let submit_password_return = self.submit_password(&mut session_conn, password).await?;

        HoneyIdResult::Ok(submit_password_return.accessToken)
    }

    /// Calls [`SubmitUsername`] endpoint of `honey.id` with [`configured`]
    /// `appPublicId` using provided [`HoneyIdConnection`] and `username`.
    ///
    /// [`SubmitUsername`] should be used to start `honey.id` authentication
    /// process using [`public`] connection.
    ///
    /// [`SubmitUsername`]: SubmitUsernameRequest
    /// [`configured`]: crate::HoneyIdConfig
    /// [`public`]: crate::enums::UserRole::Public
    pub async fn submit_username(
        &self,
        username: &str,
    ) -> HoneyIdResult<(HoneySubmitUsernameResponse, HoneyIdConnection)> {
        let mut conn = self.connect_public().await?;

        conn.send_request(
            HoneyEndpointMethodCode::SubmitUsername,
            HoneySubmitUsernameRequest {
                appPublicId: self.config.app_public_id,
                username: username.to_string(),
            },
        )
        .await?;

        let response = conn
            .receive_response::<HoneySubmitUsernameResponse>()
            .await?;

        HoneyIdResult::Ok((response, conn))
    }

    pub async fn submit_password(
        &self,
        session_conn: &mut HoneyIdConnection,
        password: &str,
    ) -> HoneyIdResult<HoneySubmitPasswordResponse> {
        session_conn
            .send_request(
                HoneyEndpointMethodCode::SubmitPassword,
                HoneySubmitPasswordRequest {
                    password: password.to_string(),
                },
            )
            .await?;

        let response = session_conn
            .receive_response::<HoneySubmitPasswordResponse>()
            .await?;

        HoneyIdResult::Ok(response)
    }

    pub async fn raw_connect(addr: &Url, header: &str) -> HoneyIdResult<HoneyIdConnection> {
        HoneyIdConnection::connect(addr, Some(header)).await
    }

    pub async fn connect_public(&self) -> HoneyIdResult<HoneyIdConnection> {
        let auth_endpoint_name = HoneyEndpointMethodCode::PublicConnect
            .schema()
            .name
            .to_lowercase();
        let header = format!("0{auth_endpoint_name}");

        Self::raw_connect(&self.config.addr, &header).await
    }
}
