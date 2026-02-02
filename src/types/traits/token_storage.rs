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

use crate::types::entity::UserPublicId;
use crate::types::traits::TokenStorage;

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