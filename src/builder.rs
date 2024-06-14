use crate::imt::Imt;
use crate::prelude::*;

const MAX_TREE_HEIGHT: usize = 32;

pub struct Builder {
    height: usize,
}

impl Builder {
    pub fn height(mut self, height: usize) -> Result<Self> {
        match height {
            1..=MAX_TREE_HEIGHT => {
                self.height = height;
                Ok(self)
            }
            _ => Err(Error::InvalidHeight(height)),
        }
    }

    pub fn build(self) -> Imt {
        Imt {
            left_digests_per_level: vec![],
            zero_digests_per_level: vec![],
            root_digest: vec![],
            max_leaves: 0,
            next_leaf_index: 0,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            height: MAX_TREE_HEIGHT,
        }
    }
}
