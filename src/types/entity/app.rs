use derive_more::{From, Into};
use uuid::Uuid;
use worktable::prelude::SizeMeasurable;
use worktable::prelude::align;
use worktable::prelude::{MemStat, SizeMeasure};

/// Public identifier for an [`App`] in the `honey.id`.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
    derive_more::Into,
    MemStat,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    SizeMeasure,
)]
#[rkyv(compare(PartialEq), derive(Debug, PartialOrd, PartialEq, Eq, Ord))]
pub struct AppPublicId(Uuid);
