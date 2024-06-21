// Re-export error type
pub use crate::error::Error;

// Result type using crate's error type
pub type Result<T> = core::result::Result<T, Error>;
