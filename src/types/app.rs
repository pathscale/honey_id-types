use derive_more::{From, Into};
use uuid::Uuid;

/// Public identifier for an [`App`] in the `honey.id`.
#[derive(Copy, Clone, Debug, Default, From, Into)]
pub struct AppPublicId(Uuid);