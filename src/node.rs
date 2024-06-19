use crate::prelude::*;
use core::ops::Index;
use sha256::digest as sha2;
use std::fmt::Display;
use std::slice::SliceIndex;

pub trait Node: Clone + Default + From<String> + Display {
    fn digest(&self, other: &Self) -> Self;
}

#[derive(Clone)]

pub struct Sha256Node {
    inner: [u8; Self::SIZE],
}

impl Sha256Node {
    const SIZE: usize = 32;
    const DSIZE: usize = 64;
}

impl Node for Sha256Node {
    fn digest(&self, other: &Self) -> Self {
        let bytes = [&self.inner[..], &other.inner[..]].concat();

        let digest = sha2(bytes);
        let bytes = hex::decode(digest).expect("Result of sha2 is valid hex");
        let mut inner = [0; Self::SIZE];
        inner.copy_from_slice(&bytes);
        Self { inner }
    }
}

impl Display for Sha256Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.inner))
    }
}

impl From<String> for Sha256Node {
    fn from(s: String) -> Self {
        let digest = sha2(s);
        let bytes = hex::decode(digest).expect("Result of sha2 is valid hex");
        let mut inner = [0; Self::SIZE];
        inner.copy_from_slice(&bytes);
        Self { inner }
    }
}

impl TryFrom<&str> for Sha256Node {
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

impl Default for Sha256Node {
    fn default() -> Self {
        Self {
            inner: [0; Self::SIZE],
        }
    }
}

impl<Idx> Index<Idx> for Sha256Node
where
    Idx: SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.inner[index]
    }
}
