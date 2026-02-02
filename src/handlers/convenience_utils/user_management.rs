use crate::id_entities::UserPublicId;

// TODO: Do we need this?

/// Defines the basic API needed by authentication
/// methods for proper role assignments.
pub trait UserStorage {
    fn get_role_by_pub_id(&self, user_pub_id: UserPublicId) -> eyre::Result<u32>;
    fn get_public_role(&self) -> u32;
    fn get_honey_auth_role(&self) -> u32;
}
