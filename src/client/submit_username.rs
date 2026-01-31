use crate::client::HoneyIdClient;
use crate::enums::EndpointMethodCode;
use crate::error::HoneyIdResult;
use crate::public_endpoints::auth_flow::SubmitUsernameRequest;
use crate::ws::HoneyIdConnection;

impl HoneyIdClient {
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
        conn: &mut HoneyIdConnection,
        username: String,
    ) -> HoneyIdResult<()> {
        conn.send_request(
            EndpointMethodCode::SubmitUsername,
            SubmitUsernameRequest {
                appPublicId: self.config.app_public_id,
                username,
            },
        )
        .await?;

        Ok(())
    }
}
