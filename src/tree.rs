use crate::height::Height;
use crate::prelude::*;
use core::ops::Index;
use sha256::digest;
use std::fmt::Display;
use std::slice::SliceIndex;

#[derive(Clone)]
pub struct Node {
    inner: [u8; Self::SIZE],
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.inner))
    }
}

impl TryFrom<&str> for Node {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s.len() {
            Self::DSIZE => {
                let bytes = hex::decode(s).map_err(|_| Error::InvalidDigest(s.len()))?;
                let mut inner = [0; Self::SIZE];
                inner.copy_from_slice(&bytes);
                Ok(Self { inner })
            }
            Self::SIZE => {
                let bytes = s.to_owned().into_bytes();
                let mut inner = [0; Self::SIZE];
                inner.copy_from_slice(&bytes);
                Ok(Self { inner })
            }
            _ => Err(Error::InvalidDigest(s.len())),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            inner: [0; Self::SIZE],
        }
    }
}

impl<Idx> Index<Idx> for Node
where
    Idx: SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.inner[index]
    }
}

impl Node {
    const SIZE: usize = 32;
    const DSIZE: usize = 64;
}

pub struct Tree {
    pub(crate) left_digests_per_level: Vec<Node>,
    pub(crate) zero_digests_per_level: Vec<Node>,
    pub(crate) root_digest: Node,
    pub(crate) height: Height,
    pub(crate) max_leaves: u64,
    pub(crate) next_leaf_index: u64,
}

impl Tree {
    pub fn root_digest(&self) -> &Node {
        &self.root_digest
    }

    /// Add a leaf to the tree.
    /// The tree's root digest and list of left-sided digests are updated.
    pub fn add_leaf(&mut self, leaf: &str) -> Result<()> {
        // Handle overflow
        if self.next_leaf_index >= self.max_leaves {
            return Err(Error::TreeOverflow(self.max_leaves));
        }

        // Continue from the last leaf index (L/R)
        let mut left_right_index = self.next_leaf_index;
        // Hash the leaf input
        let mut latest_digest: Node = digest(leaf).as_str().try_into()?;

        // Iterate over the levels of the tree
        for level in 0..self.height.into() {
            // Get the pair of tree nodes to hash together
            let (left, right) = if left_right_index % 2 == 0 {
                // Store the latest digest as the left node for the current level
                self.left_digests_per_level[level] = latest_digest.clone();
                (
                    // Left node is the latest digest we generated.
                    &latest_digest,
                    // Right node is the zero digest for this level.
                    &self.zero_digests_per_level[level],
                )
            } else {
                (
                    // Left node is either a zero digest or the latest digest from the previous level.
                    &self.left_digests_per_level[level],
                    // Right node is the latest digest we generated.
                    &latest_digest,
                )
            };
            // Hash the tree nodes
            latest_digest = digest([&left[..], &right[..]].concat())
                .as_str()
                .try_into()?;
            // Iterate up a level (towards the root node)
            left_right_index /= 2;
        }

        // Store the new root digest
        self.root_digest = latest_digest;
        // The next leaf will be on the opposite side of the current leaf (L/R)
        self.next_leaf_index += 1;

        Ok(())
    }
}
