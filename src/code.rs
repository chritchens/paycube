//! Generic string code used in account identifiers.

use crate::error::Error;
use crate::result::Result;
use arraystring::{prelude::Capacity, ArrayString};
use serde::{Deserialize, Serialize};
use std::char;

/// [`Code`] is a fixed length array string.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Code<N: Capacity>(ArrayString<N>);

impl<N: Capacity> Code<N> {
    /// `new` creates a new `Code<N>` from a string code
    pub fn new(code: &str) -> Result<Code<N>> {
        if !Code::<N>::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(Code::<N>::from_str(code))
    }

    /// `from_str` converts a string slice to a `Code<N>`
    pub fn from_str(code: &str) -> Code<N> {
        Code::<N>(ArrayString::<N>::from(code))
    }

    /// `as_str` returns the `Code<N>` as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        (code.len() as u8) == ArrayString::<N>::capacity()
            && code.find(|c: char| c.is_lowercase() || !c.is_ascii()) == None
    }

    /// `validate` validates the `Code`.
    pub fn validate(&self) -> Result<()> {
        if !Code::<N>::is_valid(&self.0) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }
}

impl<'a, N: Capacity> From<&'a str> for Code<N> {
    fn from(code: &str) -> Self {
        Code::<N>::from_str(code)
    }
}
