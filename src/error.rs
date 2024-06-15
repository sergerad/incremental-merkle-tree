#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Generic(&'static str),
    #[error("Invalid tree height: {0}")]
    InvalidHeight(usize),
    #[error("Invalid digest length: {0}")]
    InvalidDigest(usize),
}
