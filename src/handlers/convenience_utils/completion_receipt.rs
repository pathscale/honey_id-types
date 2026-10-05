//! Revocation-aware idempotency contract for completion-aware ReceiveToken callbacks.
use async_trait::async_trait;
use std::fmt;

use crate::id_entities::{AppPublicId, AuthToken, UserPublicId};

/// Stable identifier for one logical ReceiveToken completion.
///
/// Producers must reuse the same ID when retrying one completion. IDs are
/// restricted to 1-128 ASCII letters, digits, hyphens, underscores, periods,
/// or colons so adapters can use the value safely as a bounded storage key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CompletionId(String);

impl CompletionId {
    pub fn parse(value: &str) -> Result<Self, InvalidCompletionId> {
        let valid = !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_.:".contains(&byte));
        valid.then(|| Self(value.to_owned())).ok_or(InvalidCompletionId)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidCompletionId;

/// Receipt uniqueness is scoped by the configured receiver app and stable ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ReceiveTokenCompletionKey {
    pub app_pub_id: AppPublicId,
    pub completion_id: CompletionId,
}

/// ReceiveToken data used to produce a stable, versioned fingerprint input.
///
/// The token is a bearer credential. Adapters must hash the fingerprint input
/// and persist only the digest; they must not log or retain this input or token.
pub struct ReceiveTokenCompletionPayload {
    pub token: AuthToken,
    pub username: String,
    pub user_pub_id: UserPublicId,
}

impl ReceiveTokenCompletionPayload {
    /// Canonical bytes for SHA-256 fingerprinting by the durable adapter.
    /// Each UTF-8 component is length-prefixed with an unsigned big-endian
    /// 64-bit length to avoid ambiguous concatenations.
    pub fn fingerprint_material(&self) -> Vec<u8> {
        let token = self.token.to_string();
        let user_pub_id = self.user_pub_id.to_string();
        let mut material = b"honey.id.ReceiveTokenCompletionPayload.v1\0".to_vec();
        append_component(&mut material, token.as_bytes());
        append_component(&mut material, self.username.as_bytes());
        append_component(&mut material, user_pub_id.as_bytes());
        material
    }
}

fn append_component(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&(value.len() as u64).to_be_bytes());
    output.extend_from_slice(value);
}

/// Durable adapter state for a receipt key. `Pending` can be resumed only with
/// the same payload fingerprint; `Completed` is final and must never return to
/// `Pending`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompletionReceiptState {
    Pending,
    Completed,
}

/// The adapter completed or resumed a pending operation, or found its durable
/// receipt already completed. `AlreadyCompleted` must not reapply user/token
/// effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiveTokenCompletionDisposition {
    Applied,
    AlreadyCompleted,
}

#[derive(Debug)]
pub enum CompletionReceiptError {
    /// The same app and completionId were already used for another payload.
    Conflict,
    /// A durable user/app revocation fence exists for this payload.
    Revoked,
    /// The configured adapter does not implement the safe completion protocol.
    Unsupported,
    /// Durable storage or an idempotent side effect failed.
    Storage(eyre::Report),
}

impl fmt::Display for CompletionReceiptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict => formatter.write_str("completionId conflicts with a committed payload"),
            Self::Revoked => formatter.write_str("user has been revoked for this app"),
            Self::Unsupported => formatter.write_str("revocation-aware completion storage is not configured"),
            Self::Storage(error) => write!(formatter, "completion storage failed: {error}"),
        }
    }
}

impl std::error::Error for CompletionReceiptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Storage(error) => Some(error.as_ref()),
            Self::Conflict | Self::Revoked | Self::Unsupported => None,
        }
    }
}

/// Required adapter for opting into completion-aware callbacks.
///
/// A conforming implementation has to coordinate its durable receipt state,
/// user mutation, token mutation, and revocation fence. It must persist a
/// digest-only `Pending` receipt before effects; a same-fingerprint retry of a
/// pending receipt resumes idempotently; a different fingerprint conflicts.
/// Once all effects are complete it durably changes the receipt to `Completed`
/// and retains that receipt for as long as its ID may be retried.
/// A duplicate completed receipt returns `AlreadyCompleted` without applying
/// effects. In particular it must never recreate a token or user removed since
/// the original completion.
///
/// `revoke_user` installs a durable token/user tombstone before finishing
/// deletion. `None` app scope means the revocation covers every app for that
/// user. Apply and revoke must serialize on a durable per-user fence that
/// covers every side effect: if revoke wins, pending work fails with `Revoked`;
/// if apply wins, revoke subsequently removes its effects. A tombstone is
/// permanent for the relevant scope. Retries of an interrupted revoke resume
/// deletion.
///
/// There is no implied transaction across independent stores. An adapter may
/// use a transaction if all state is in one transactional store; otherwise it
/// must use durable fencing and operation-idempotent/recoverable effects. An
/// adapter that cannot guarantee these properties must keep the default
/// `Unsupported` behavior. The current volatile `TokenWorkTableStorage` is not
/// such an adapter.
#[async_trait]
pub trait ReceiveTokenCompletionStorage: Send + Sync {
    /// Applies or resumes a completion. `fingerprint_material` must be hashed
    /// with SHA-256 and only the digest persisted. Never log or persist the raw
    /// token/fingerprint material. `payload` is needed for the idempotent user
    /// and token effects; its raw token must not enter receipt storage.
    async fn apply_receive_token(
        &self,
        _key: ReceiveTokenCompletionKey,
        _fingerprint_material: &[u8],
        _payload: &ReceiveTokenCompletionPayload,
    ) -> Result<ReceiveTokenCompletionDisposition, CompletionReceiptError> {
        Err(CompletionReceiptError::Unsupported)
    }

    /// Records revocation and finishes deleting app/user data. All user/token
    /// deletion paths for users accepted by this adapter must call this method;
    /// none may bypass the tombstone. This must use the same durable fence as
    /// `apply_receive_token`.
    async fn revoke_user(
        &self,
        _app_pub_id: Option<AppPublicId>,
        _user_pub_id: UserPublicId,
    ) -> Result<(), CompletionReceiptError> {
        Err(CompletionReceiptError::Unsupported)
    }
}
