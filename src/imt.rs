use crate::prelude::*;

type Node = Vec<u8>;
type Branch = Vec<Node>;

pub struct Imt {
    pub(crate) left_digests_per_level: Branch,
    pub(crate) zero_digests_per_level: Branch,
    pub(crate) root_digest: Node,
    pub(crate) max_leaves: usize,
    pub(crate) next_leaf_index: usize,
}
