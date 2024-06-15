use crate::imt::Imt;
use crate::imt::Node;
use crate::prelude::*;
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

    pub fn build(self) -> Result<Imt> {
        let empty_digests: Vec<Node> = vec![Node::default(); self.height.into()];
        let zero_digests: Vec<Node> = empty_digests
            .iter()
            .skip(1)
            .scan(Node::default(), |acc: &mut Node, _| {
                let digest = digest([&acc[..], &acc[..]].concat());
                println!("{digest}");
                Some(Node::try_from(digest).expect("todo123"))
            })
            .collect();
        Ok(Imt {
            left_digests_per_level: vec![Node::default(); self.height.into()],
            root_digest: zero_digests
                .last()
                .ok_or(Error::Generic("no last digest"))?
                .clone(),
            zero_digests_per_level: zero_digests,
            max_leaves: 2_u64.pow(self.height.into()),
            next_leaf_index: 0,
        })
    }
}
