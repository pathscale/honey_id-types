use crate::client::HoneyIdClient;
use crate::enums::EndpointMethodCode;
use crate::error::HoneyIdResult;
use crate::public_endpoints::auth_flow::StartAuthRequest;
use crate::ws::HoneyIdConnection;

impl HoneyIdClient {
    /// Calls [`StartAuth`] endpoint of `honey.id` with [`configured`]
    /// `appPublicId` using provided [`HoneyIdConnection`].
    ///
    /// [`StartAuth`] should be used to start `honey.id` authentication process
    /// using [`public`] connection.
    ///
    /// [`StartAuth`]: StartAuthRequest
    /// [`configured`]: crate::HoneyIdConfig
    /// [`public`]: crate::enums::UserRole::Public
    pub async fn start_auth(&self, conn: &mut HoneyIdConnection) -> HoneyIdResult<()> {
        conn.send_request(
            EndpointMethodCode::StartAuth,
            StartAuthRequest {
                appPublicId: self.config.app_public_id,
            },
        )
            .await?;

        Ok(())
    }
}