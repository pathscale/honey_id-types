use derive_more::{Display, From, Into};
use psc_nanoid::{Nanoid, alphabet::Base62Alphabet};
use rkyv::{Archive, Deserialize, Serialize};
use worktable::prelude::{MemStat, SizeMeasurable, align};

/// Public identifier for an [`App`] in the `honey.id`.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Display, From, Into, Archive, Deserialize, Serialize,
)]
#[rkyv(derive(Debug))]
pub struct AppPublicId(Nanoid<16, Base62Alphabet>);

impl Default for AppPublicId {
    fn default() -> Self {
        Self(Nanoid::new())
    }
}

impl MemStat for AppPublicId {
    fn heap_size(&self) -> usize {
        0
    }

    fn used_size(&self) -> usize {
        16
    }
}

impl SizeMeasurable for AppPublicId {
    fn aligned_size(&self) -> usize {
        align(16)
    }
}

/// Public identifier for a [`User`] in the `honey.id`.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Display, From, Into, Archive, Deserialize, Serialize,
)]
#[rkyv(derive(Debug))]
pub struct UserPublicId(Nanoid<16, Base62Alphabet>);

impl Default for UserPublicId {
    fn default() -> Self {
        Self(Nanoid::new())
    }
}

impl MemStat for UserPublicId {
    fn heap_size(&self) -> usize {
        0
    }

    fn used_size(&self) -> usize {
        16
    }
}

impl SizeMeasurable for UserPublicId {
    fn aligned_size(&self) -> usize {
        align(16)
    }
}
