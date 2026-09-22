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

/// An authentication token minted by `honey.id` and presented to an app backend.
///
/// A nanoid rather than a UUID. It is the same 16 characters of Base62 as every
/// other identifier in this crate, which is 95 bits of entropy against a v4
/// UUID's 122; both are far past guessing a bearer token, and the uniformity is
/// worth more than the 27 bits. It also packs to 12 bytes through the same
/// `PackedNanoid` path the other ids use, so a token in a WorkTable row costs
/// what a UUID did and every id in the fleet is now one kind of thing.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Display, From, Into, Archive, Deserialize, Serialize,
)]
#[rkyv(derive(Debug))]
pub struct AuthToken(Nanoid<16, Base62Alphabet>);

impl Default for AuthToken {
    fn default() -> Self {
        Self(Nanoid::new())
    }
}

impl AuthToken {
    /// Mint a fresh token.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::str::FromStr for AuthToken {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Nanoid::try_from_str(s)
            .map(Self)
            .map_err(|_| eyre::eyre!("invalid auth token: expected a 16 character Base62 nanoid"))
    }
}

impl MemStat for AuthToken {
    fn heap_size(&self) -> usize {
        0
    }

    fn used_size(&self) -> usize {
        16
    }
}

impl SizeMeasurable for AuthToken {
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
