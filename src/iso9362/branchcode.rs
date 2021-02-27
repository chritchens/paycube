//! BIC's branch code, as defined by ISO 9362.

use crate::code::Code;
use crate::error::Error;
use crate::result::Result;
use arraystring::typenum::U3;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::result::Result as StdResult;
use std::str::FromStr;

/// `PRIMARY_BRANCH_CODE` is the branch code of the primary office
pub const PRIMARY_BRANCH_CODE: &str = "XXX";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BranchCode(Code<U3>);

impl BranchCode {
    pub fn new(code: &str) -> Result<BranchCode> {
        if !BranchCode::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(BranchCode(Code::<U3>::from(code)))
    }

    /// `as_str` returns the `BranchCode` as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        Code::<U3>::is_valid(code)
    }

    /// `validate` validates the [`BranchCode`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !BranchCode::is_valid(self.as_str()) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }

    /// `is_primary` returns if the branch is the primary office.
    pub fn is_primary(&self) -> bool {
        Code::<U3>::from(PRIMARY_BRANCH_CODE) == self.0
    }
}

impl FromStr for BranchCode {
    type Err = Error;

    fn from_str(code: &str) -> StdResult<Self, Self::Err> {
        BranchCode::new(code)
    }
}

impl<'a> From<&'a str> for BranchCode {
    fn from(code: &str) -> Self {
        BranchCode::from_str(code).unwrap()
    }
}

impl Serialize for BranchCode {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> StdResult<S::Ok, S::Error> {
        Serialize::serialize(self.0.as_str(), ser)
    }
}

impl<'a> Deserialize<'a> for BranchCode {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> StdResult<Self, D::Error> {
        <&str>::deserialize(des).map(Self::from)
    }
}

mod test {
    #[allow(unused_imports)] // TODO
    use super::{BranchCode, Result};

    #[test]
    fn is_valid() {
        const WRONG_CODES: &[&str] = &["abc", "AB", "0xAA", "ZZ!!"];
        const VALID_CODES: &[&str] = &["AAA", "DEU", "XYZ", "XXX"];

        for code in WRONG_CODES.iter() {
            assert!(!BranchCode::is_valid(code));
        }

        for code in VALID_CODES.iter() {
            assert!(BranchCode::is_valid(code));
        }
    }
}
