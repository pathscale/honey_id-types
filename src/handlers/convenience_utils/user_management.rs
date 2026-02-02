use async_trait::async_trait;

use crate::{
    endpoints::callback::{
        HoneyReceiveTokenRequest, HoneyReceiveTokenResponse, HoneyReceiveUserInfoRequest,
        HoneyReceiveUserInfoResponse,
    },
    id_entities::UserPublicId,
};

// TODO: Do we need this?

/// Defines the basic API needed by authentication
/// methods for proper role assignments.
#[async_trait]
pub trait UserStorage {
    fn get_api_role_by_pub_id(&self, user_pub_id: UserPublicId) -> eyre::Result<u32>;
    fn get_public_role(&self) -> u32;
    fn get_honey_auth_role(&self) -> u32;
    async fn create_or_update_user(
        &self,
        user_info_request: HoneyReceiveTokenRequest,
    ) -> eyre::Result<HoneyReceiveTokenResponse>;
}
