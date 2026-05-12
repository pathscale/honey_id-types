use derive_more::{Display, From, Into};
use psc_nanoid::{
    Nanoid,
    alphabet::Base62Alphabet,
    packed::{PackError, PackedNanoid},
};
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

impl AppPublicId {
    /// Pack the ID into a compact 12-byte representation.
    ///
    /// # Errors
    ///
    /// Returns `PackError` if the ID contains invalid characters (should never happen for valid IDs).
    pub fn pack(&self) -> Result<PackedNanoid<16, 12, Base62Alphabet>, PackError> {
        PackedNanoid::pack(&self.0)
    }

    /// Unpack a compact byte representation back into an `AppPublicId`.
    ///
    /// # Errors
    ///
    /// Returns `PackError` if the packed data contains invalid indices.
    pub fn unpack(packed: PackedNanoid<16, 12, Base62Alphabet>) -> Result<Self, PackError> {
        packed.unpack().map(Self)
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

impl UserPublicId {
    /// Pack the ID into a compact 12-byte representation.
    ///
    /// # Errors
    ///
    /// Returns `PackError` if the ID contains invalid characters (should never happen for valid IDs).
    pub fn pack(&self) -> Result<PackedNanoid<16, 12, Base62Alphabet>, PackError> {
        PackedNanoid::pack(&self.0)
    }

    /// Unpack a compact byte representation back into a `UserPublicId`.
    ///
    /// # Errors
    ///
    /// Returns `PackError` if the packed data contains invalid indices.
    pub fn unpack(packed: PackedNanoid<16, 12, Base62Alphabet>) -> Result<Self, PackError> {
        packed.unpack().map(Self)
    }
}
