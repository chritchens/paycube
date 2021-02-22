use crate::error::Error;
use std::result;

/// `Result` type of the crate.
pub type Result<T> = result::Result<T, Error>;
