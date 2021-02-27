//! BIC's (non-)financial institution code, as defined by ISO 9362.

use crate::code::Code;
use crate::error::Error;
use crate::result::Result;
use arraystring::typenum::U4;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstitutionCode(Code<U4>);

impl InstitutionCode {
    pub fn new(code: &str) -> Result<InstitutionCode> {
        if !InstitutionCode::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(InstitutionCode(Code::<U4>::from(code)))
    }

    /// `as_str` returns the `InstitutionCode` as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        Code::<U4>::is_valid(code) && code.find(|c: char| !c.is_ascii_alphabetic()) == None
    }

    /// `validate` validates the [`InstitutionCode`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !InstitutionCode::is_valid(self.as_str()) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }
}

impl FromStr for InstitutionCode {
    type Err = Error;

    fn from_str(code: &str) -> StdResult<Self, Self::Err> {
        InstitutionCode::new(code)
    }
}

impl<'a> From<&'a str> for InstitutionCode {
    fn from(code: &str) -> Self {
        InstitutionCode::from_str(code).unwrap()
    }
}

impl Serialize for InstitutionCode {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> StdResult<S::Ok, S::Error> {
        Serialize::serialize(self.0.as_str(), ser)
    }
}

impl<'a> Deserialize<'a> for InstitutionCode {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> StdResult<Self, D::Error> {
        <&str>::deserialize(des).map(Self::from)
    }
}

mod test {
    #[allow(unused_imports)] // TODO
    use super::{InstitutionCode, Result};

    #[test]
    fn is_valid() {
        const WRONG_CODES: &[&str] = &["abc", "AB", "0XAA", "ZZ!!"];
        const VALID_CODES: &[&str] = &["AAAA", "DEUT", "WXYZ", "XXXX"];

        for code in WRONG_CODES.iter() {
            assert!(!InstitutionCode::is_valid(code));
        }

        for code in VALID_CODES.iter() {
            assert!(InstitutionCode::is_valid(code));
        }
    }
}
