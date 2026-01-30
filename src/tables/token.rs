//! Token storage [`WorkTable`].
//!
//! This module defines the in-memory [`TokenWorkTable`] for storing
//! authentication tokens and their associated [`UserPublicId`].
//!
//! Tokens are not persisted, so `User`'s should re-login on app restart.

use uuid::Uuid;
use worktable::prelude::*;
use worktable::worktable;

use crate::types::entity::UserPublicId;

worktable!(
    name: Token,
    persist: false,
    columns: {
        id: u64 primary_key autoincrement,
        public_id: UserPublicId,
        token: Uuid,
    },
    indexes: {
        public_id: public_id unique,
    }
);
