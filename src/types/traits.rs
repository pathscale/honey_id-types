mod token_storage;

use uuid::Uuid;

use crate::types::entity::UserPublicId;

pub use token_storage::TokenWorkTableStorage;

/// Describes API of [`TokenStorage`], which allows to store and validate
/// returned in callbacks `User` `token`.
pub trait TokenStorage {
    /// Stores received `token` which belongs to `User` with provided
    /// [`UserPublicId`].
    fn store_token(&self, user_pub_id: UserPublicId, token: Uuid) -> eyre::Result<()>;
    /// Validates provided `token` and returns [`UserPublicId`] if `token` is
    /// valid. Errors otherwise.
    fn validate_token(&self, token: Uuid) -> eyre::Result<UserPublicId>;
}

/// Describes API of [`UserStorage`], which defines API needed by authentication
/// methods for proper role assignments.
pub trait UserStorage {
    fn get_role_by_pub_id(&self, user_pub_id: UserPublicId) -> eyre::Result<u32>;
}