#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Generic error that we can replace enetually
    #[error("Generic {0}")]
    Generic(String),

    #[error("Invalid tree height: {0}")]
    InvalidHeight(usize),
}
