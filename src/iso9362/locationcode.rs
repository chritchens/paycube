//! BIC's location code, as defined by ISO 9362.

use crate::code::Code;
use crate::error::Error;
use crate::result::Result;
use arraystring::typenum::U2;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::result::Result as StdResult;
use std::str::FromStr;

/// `PASSIVE_LOCATION_SUFFIX` is the suffix used by locations
/// not connected to the SWIFT network
pub const PASSIVE_LOCATION_SUFFIX: char = '1';

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocationCode(Code<U2>);

impl LocationCode {
    pub fn new(code: &str) -> Result<LocationCode> {
        if !LocationCode::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(LocationCode(Code::<U2>::from(code)))
    }

    /// `as_str` returns the `LocationCode` as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        Code::<U2>::is_valid(code)
    }

    /// `validate` validates the [`LocationCode`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !LocationCode::is_valid(self.as_str()) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }

    /// `is_connected` returns if the location is connected to the
    /// SWIFT network.
    pub fn is_connected(&self) -> bool {
        self.as_str().chars().nth(1).unwrap() != PASSIVE_LOCATION_SUFFIX
    }
}

impl FromStr for LocationCode {
    type Err = Error;

    fn from_str(code: &str) -> StdResult<Self, Self::Err> {
        LocationCode::new(code)
    }
}

impl<'a> From<&'a str> for LocationCode {
    fn from(code: &str) -> Self {
        LocationCode::from_str(code).unwrap()
    }
}

impl Serialize for LocationCode {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> StdResult<S::Ok, S::Error> {
        Serialize::serialize(self.0.as_str(), ser)
    }
}

impl<'a> Deserialize<'a> for LocationCode {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> StdResult<Self, D::Error> {
        <&str>::deserialize(des).map(Self::from)
    }
}

mod test {
    #[allow(unused_imports)] // TODO
    use super::{LocationCode, Result};

    #[test]
    fn is_valid() {
        const WRONG_CODES: &[&str] = &["abc", "ab", "1234", "1", "!"];
        const VALID_CODES: &[&str] = &["AA", "B0", "C1", "23"];

        for code in WRONG_CODES.iter() {
            assert!(!LocationCode::is_valid(code));
        }

        for code in VALID_CODES.iter() {
            assert!(LocationCode::is_valid(code));
        }
    }
}
