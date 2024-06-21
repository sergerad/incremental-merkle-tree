use digest::{Digest, Output};

use crate::prelude::*;
use crate::tree::Tree;

use crate::height::Height;

/// Builder for the [`Tree`] type.
#[derive(Default)]
pub struct Builder {
    height: Height,
}

impl Builder {
    /// Set the height of the tree.
    pub fn height(mut self, height: Height) -> Self {
        self.height = height;
        self
    }

    /// Build the tree with a specific hasher.
    pub fn build<D: Digest>(self) -> Result<Tree<D>> {
        // Create the list of digests generated from zeroed leaves
        // for each level in the tree
        let mut default_nodes = vec![Output::<D>::default()];
        for level in 1..self.height.into() {
            let previous_node = &default_nodes[level - 1];
            let node = D::new()
                .chain_update(previous_node)
                .chain_update(previous_node)
                .finalize();
            default_nodes.push(node);
        }

        // Initialize the tree from the last digest in the zero_digests list
        Ok(Tree {
            root_node: default_nodes
                .last()
                .ok_or(Error::Generic("No last digest"))?
                .clone(),
            left_nodes: vec![Output::<D>::default(); self.height.into()],
            default_nodes,
            height: self.height,
            max_leaves: 2_u64.pow(self.height.into()),
            next_leaf_index: 0,
            _digest: std::marker::PhantomData,
        })
    }
}
