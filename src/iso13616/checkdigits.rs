//! IBAN's check digits, as defined by ISO 13616.

use crate::code::Code;
use crate::error::Error;
use crate::result::Result;
use arraystring::typenum::U2;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CheckDigits(Code<U2>);

impl CheckDigits {
    pub fn new(code: &str) -> Result<CheckDigits> {
        if !CheckDigits::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(CheckDigits(Code::<U2>::from(code)))
    }

    /// `as_str` returns the `CheckDigits` as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        Code::<U2>::is_valid(code) && code.find(|c: char| !c.is_ascii_digit()) == None
    }

    /// `validate` validates the [`CheckDigits`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !CheckDigits::is_valid(self.as_str()) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }
}

impl FromStr for CheckDigits {
    type Err = Error;

    fn from_str(code: &str) -> StdResult<Self, Self::Err> {
        CheckDigits::new(code)
    }
}

impl<'a> From<&'a str> for CheckDigits {
    fn from(code: &str) -> Self {
        CheckDigits::from_str(code).unwrap()
    }
}

impl Serialize for CheckDigits {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> StdResult<S::Ok, S::Error> {
        Serialize::serialize(self.0.as_str(), ser)
    }
}

impl<'a> Deserialize<'a> for CheckDigits {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> StdResult<Self, D::Error> {
        <&str>::deserialize(des).map(Self::from)
    }
}

mod test {
    #[allow(unused_imports)] // TODO
    use super::{CheckDigits, Result};

    #[test]
    fn is_valid() {
        const WRONG_CODES: &[&str] = &["abc", "AB", "0xAA", "CODE", "ZZ"];
        const VALID_CODES: &[&str] = &["00", "01", "10", "11", "99"];

        for code in WRONG_CODES.iter() {
            assert!(!CheckDigits::is_valid(code));
        }

        for code in VALID_CODES.iter() {
            assert!(CheckDigits::is_valid(code));
        }
    }
}
