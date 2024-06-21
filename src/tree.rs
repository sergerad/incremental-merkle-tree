use digest::{Digest, Output};

use crate::height::Height;
use crate::prelude::*;

/// Incremental Merkle tree type.
/// Has a fixed height and maintains a pair of digests for each non-root level of the tree.
/// Root digest is updated as leaves are added.
/// Use the [`crate::builder::Builder`] to create a new tree.
pub struct Tree<D: Digest> {
    pub(crate) left_nodes: Vec<Output<D>>,
    pub(crate) default_nodes: Vec<Output<D>>,
    pub(crate) root_node: Output<D>,
    pub(crate) height: Height,
    pub(crate) max_leaves: u64,
    pub(crate) next_leaf_index: u64,
    pub(crate) _digest: std::marker::PhantomData<D>,
}

impl<D: Digest> Tree<D> {
    /// Get the root digest of the tree.
    pub fn root(&self) -> Output<D> {
        self.root_node.clone()
    }

    /// Add a leaf to the tree.
    /// The tree's root digest and list of left-sided digests are updated.
    pub fn add_leaf(&mut self, leaf: impl AsRef<[u8]>) -> Result<()> {
        // Handle overflow
        if self.next_leaf_index >= self.max_leaves {
            return Err(Error::TreeOverflow(self.max_leaves));
        }

        // Continue from the last leaf index (L/R)
        let mut left_right_index = self.next_leaf_index;
        let mut latest_node = D::digest(leaf.as_ref());

        // Iterate over the levels of the tree
        for level in 0..self.height.into() {
            // Get the pair of tree nodes to hash together
            let (left, right) = if left_right_index % 2 == 0 {
                // Store the latest digest as the left node for the current level
                self.left_nodes[level] = latest_node.clone();
                (
                    // Left node is the latest digest we generated.
                    &latest_node,
                    // Right node is the zero digest for this level.
                    &self.default_nodes[level],
                )
            } else {
                (
                    // Left node was stored previously
                    &self.left_nodes[level],
                    // Right node is the latest digest we generated.
                    &latest_node,
                )
            };
            // Hash the tree nodes
            latest_node = D::new().chain_update(left).chain_update(right).finalize();
            // Iterate up a level (towards the root node)
            left_right_index /= 2;
        }

        // Store the new root digest
        self.root_node = latest_node;
        // The next leaf will be on the opposite side of the current leaf (L/R)
        self.next_leaf_index += 1;

        Ok(())
    }
}
