//! Definition of [`TokenWorkTableStorage`].
//!
//! This module defines the in-memory [`TokenWorkTable`] for storing
//! authentication tokens and their associated [`UserPublicId`] and
//! [`TokenWorkTableStorage`] newtype.
//!
//! Tokens are not persisted, so `User`'s should re-login on app restart.

use eyre::eyre;
use uuid::Uuid;
use worktable::prelude::*;
use worktable::worktable;

use crate::types::id_entities::UserPublicId;

/// Describes the API of [`TokenStorage`], which simplifies and abstracts the storage and validation
/// of tokens sent from Auth to App BEs.
pub trait TokenStorage {
    /// Stores received `token` which belongs to `User` with provided
    /// [`UserPublicId`].
    fn store_token(&self, user_pub_id: UserPublicId, token: Uuid) -> eyre::Result<()>;
    /// Validates provided `token` and returns User internal ID: u64, and [`UserPublicId`] if `token` is
    /// valid. Errors otherwise.
    fn validate_token(&self, token: Uuid) -> eyre::Result<UserPublicId>;
}

worktable!(
    name: Token,
    persist: false,
    columns: {
        id: u64 primary_key autoincrement,
        public_id: UserPublicId,
        token: Uuid,
    },
    indexes: {
        public_id_idx: public_id unique,
        token_idx: token unique,
    }
);

/// [`TokenWorkTable`] wrapper implementing [`TokenStorage`].
#[derive(Default)]
pub struct TokenWorkTableStorage(TokenWorkTable);

impl TokenStorage for TokenWorkTableStorage {
    fn store_token(&self, user_pub_id: UserPublicId, token: Uuid) -> eyre::Result<()> {
        self.0.insert(TokenRow {
            id: self.0.get_next_pk().into(),
            public_id: user_pub_id,
            token,
        })?;
        Ok(())
    }

    fn validate_token(&self, token: Uuid) -> eyre::Result<UserPublicId> {
        let entry = self
            .0
            .select_by_token(token)
            .ok_or_else(|| eyre!("token not found"))?;
        Ok(entry.public_id)
    }
}
