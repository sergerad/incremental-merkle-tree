use crate::prelude::*;
use crate::tree::Node;
use crate::tree::Tree;
use sha256::digest;

use crate::height::Height;

#[derive(Default)]
pub struct Builder {
    height: Height,
}

impl Builder {
    pub fn height(mut self, height: Height) -> Self {
        self.height = height;
        self
    }

    pub fn build(self) -> Result<Tree> {
        // Create the list of digests generated from zeroed leaves
        // for each level in the tree
        let mut zero_digests = vec![Node::default(); 1];
        for level in 1..self.height.into() {
            let digest =
                digest([&zero_digests[level - 1][..], &zero_digests[level - 1][..]].concat());
            zero_digests.push(
                Node::try_from(digest.as_str())
                    .expect("Failed to create tree node from digest string"),
            );
        }

        // Initialize the tree from the last digest in the zero_digests list
        Ok(Tree {
            root_digest: zero_digests
                .last()
                .ok_or(Error::Generic("No last digest"))?
                .clone(),
            left_digests_per_level: vec![Node::default(); self.height.into()],
            zero_digests_per_level: zero_digests,
            height: self.height,
            max_leaves: 2_u64.pow(self.height.into()),
            next_leaf_index: 0,
        })
    }
}
