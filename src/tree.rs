use crate::height::Height;
use crate::prelude::*;
use core::ops::Index;
use sha256::digest;
use std::slice::SliceIndex;

#[derive(Clone)]
pub struct Node {
    inner: [u8; Self::SIZE],
}

impl TryFrom<String> for Node {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        match s.len() {
            Self::DSIZE => {
                // todo
                let bytes = hex::decode(s.as_str()).map_err(|_| Error::InvalidDigest(s.len()))?;
                let mut inner = [0; Self::SIZE];
                inner.copy_from_slice(&bytes);
                Ok(Self { inner })
            }
            Self::SIZE => {
                let bytes = s.into_bytes();
                let mut inner = [0; Self::SIZE];
                inner.copy_from_slice(&bytes);
                Ok(Self { inner })
            }
            _ => Err(Error::InvalidDigest(s.len())),
        }
        //let bytes = s.into_bytes();
        //if bytes.len() != Self::SIZE {
        //    return Err(Error::InvalidDigest(bytes.len()));
        //}
        //Ok(Self {
        //    inner: bytes.try_into().expect("todo"),
        //})
    }
}
//impl From<String> for Node {
//    fn from(s: String) -> Self {
//        let bytes = s.into_bytes();
//        let mut inner = [0; Self::SIZE];
//        inner.copy_from_slice(&bytes);
//        Self { inner }
//    }
//}

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

    pub fn add_leaf(&mut self, leaf: Node) -> Result<()> {
        if self.next_leaf_index >= self.max_leaves {
            return Err(Error::TreeOverflow(self.max_leaves));
        }
        let mut left_right_index = self.next_leaf_index;
        let mut latest_digest: Node = digest(&leaf[..]).try_into()?;

        for level in 0..self.height.into() {
            let (left, right) = if left_right_index % 2 == 0 {
                self.left_digests_per_level[level] = latest_digest.clone(); // todo: no clone?
                (&latest_digest, &self.zero_digests_per_level[level])
            } else {
                (&self.left_digests_per_level[level], &latest_digest)
            };
            latest_digest = digest([&left[..], &right[..]].concat()).try_into()?;
            left_right_index /= 2;
        }

        self.root_digest = latest_digest;

        Ok(())
    }
}
