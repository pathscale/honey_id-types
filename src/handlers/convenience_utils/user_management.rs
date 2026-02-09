use async_trait::async_trait;
use uuid::Uuid;

use crate::id_entities::UserPublicId;

#[derive(Debug, Clone)]
pub struct CreateUserInfo {
    pub username: String,
    pub user_pub_id: Uuid,
    /// Required for platform app, since it needs to keep track of users of other Apps
    pub app_pub_id: Option<Uuid>,
}

/// Defines the basic API needed by authentication
/// methods for proper role assignments.
#[async_trait]
pub trait UserStorage {
    fn get_api_roles_by_pub_id(&self, user_pub_id: UserPublicId) -> eyre::Result<Vec<u32>>;
    fn get_public_roles(&self) -> &[u32];
    fn get_honey_auth_role(&self) -> u32;
    async fn create_or_update_user(&self, user_info_request: CreateUserInfo) -> eyre::Result<()>;
}
