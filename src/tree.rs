use crate::height::Height;
use crate::node::Node;
use crate::prelude::*;

pub struct Tree<N: Node> {
    pub(crate) left_digests_per_level: Vec<N>,
    pub(crate) zero_digests_per_level: Vec<N>,
    pub(crate) root_digest: N,
    pub(crate) height: Height,
    pub(crate) max_leaves: u64,
    pub(crate) next_leaf_index: u64,
}

impl<N: Node> Tree<N> {
    pub fn root_digest(&self) -> &N {
        &self.root_digest
    }

    /// Add a leaf to the tree.
    /// The tree's root digest and list of left-sided digests are updated.
    pub fn add_leaf(&mut self, leaf: N) -> Result<()> {
        // Handle overflow
        if self.next_leaf_index >= self.max_leaves {
            return Err(Error::TreeOverflow(self.max_leaves));
        }

        // Continue from the last leaf index (L/R)
        let mut left_right_index = self.next_leaf_index;
        let mut latest_digest = leaf;

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
            latest_digest = left.digest(right);
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
