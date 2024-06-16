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
        let mut zero_digests = vec![Node::default(); 1];
        for level in 1..self.height.into() {
            let digest =
                digest([&zero_digests[level - 1][..], &zero_digests[level - 1][..]].concat());
            zero_digests.push(Node::try_from(digest).expect("todo123"));
        }
        Ok(Tree {
            left_digests_per_level: vec![Node::default(); self.height.into()],
            root_digest: zero_digests
                .last()
                .ok_or(Error::Generic("no last digest"))?
                .clone(),
            zero_digests_per_level: zero_digests,
            height: self.height,
            max_leaves: 2_u64.pow(self.height.into()),
            next_leaf_index: 0,
        })
    }
}
