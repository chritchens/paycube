//! IBAN's BBAN (Basic Bank Account Number), as defined by ISO 13616.

use crate::error::Error;
use crate::result::Result;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
use std::str::FromStr;

/// `MAX_LENGTH` is a `BBAN` code maximum length.
pub const MAX_LENGTH: usize = 30;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BBAN(String);

impl BBAN {
    pub fn new(code: &str) -> Result<BBAN> {
        // NOTE: we only use upper-case codes, but a BBAN
        // can be lower/mixed-case
        let cd: &str = &code.to_uppercase();

        if !BBAN::is_valid(cd) {
            return Err(Error::InvalidCode);
        }

        Ok(BBAN(cd.into()))
    }

    /// `as_str` returns the `BBAN` as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        // NOTE: we only use upper-case codes, but a BBAN
        // can be lower/mixed-case
        code.len() <= MAX_LENGTH && code.find(|c: char| !c.is_ascii_alphanumeric()) == None
    }

    /// `validate` validates the [`BBAN`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !BBAN::is_valid(self.as_str()) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }
}

impl FromStr for BBAN {
    type Err = Error;

    fn from_str(code: &str) -> StdResult<Self, Self::Err> {
        BBAN::new(code)
    }
}

impl<'a> From<&'a str> for BBAN {
    fn from(code: &str) -> Self {
        BBAN::from_str(code).unwrap()
    }
}

mod test {
    #[allow(unused_imports)] // TODO
    use super::{Result, BBAN};

    #[test]
    fn is_valid() {
        const WRONG_CODES: &[&str] = &["!", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaah"];
        const VALID_CODES: &[&str] = &["00", "001", "ABC44", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"];

        for code in WRONG_CODES.iter() {
            assert!(!BBAN::is_valid(code));
        }

        for code in VALID_CODES.iter() {
            assert!(BBAN::is_valid(code));
        }
    }
}
