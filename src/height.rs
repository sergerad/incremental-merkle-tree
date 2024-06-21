use crate::prelude::*;

/// Height of the incremental Merkle tree.
/// Used by the [`crate::builder::Builder`] to create the [`crate::tree::Tree`].
#[derive(Clone, Copy)]
pub struct Height(usize);

impl Default for Height {
    fn default() -> Self {
        Self(Self::MAX)
    }
}

impl TryFrom<usize> for Height {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self> {
        match value {
            1..=Self::MAX => Ok(Self(value)),
            _ => Err(Error::InvalidHeight(value)),
        }
    }
}

impl From<Height> for usize {
    fn from(value: Height) -> Self {
        value.0
    }
}

impl From<Height> for u32 {
    fn from(height: Height) -> Self {
        height.0 as u32
    }
}

impl Height {
    const MAX: usize = 32;
}
