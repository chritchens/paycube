//! Country codes as defined by ISO 3166-1 alpha-2.

use crate::code::Code;
use crate::error::Error;
use crate::result::Result;
use arraystring::typenum::U2;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::collections::BTreeMap;
use std::result::Result as StdResult;
use std::str::FromStr;

/// [`CountryCode`] is an ISO 3166-1 alpha-2 code
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CountryCode(Code<U2>);

impl CountryCode {
    pub fn new(code: &str) -> Result<CountryCode> {
        if !CountryCode::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(CountryCode(Code::<U2>::from(code)))
    }

    /// `as_str` returns the `CountryCode` as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        Code::<U2>::is_valid(code) && code.find(|c: char| !c.is_ascii_alphabetic()) == None
    }

    /// `validate` validates the [`CountryCode`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !CountryCode::is_valid(self.as_str()) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }
}

impl FromStr for CountryCode {
    type Err = Error;

    fn from_str(code: &str) -> StdResult<Self, Self::Err> {
        CountryCode::new(code)
    }
}

impl<'a> From<&'a str> for CountryCode {
    fn from(code: &str) -> Self {
        CountryCode::from_str(code).unwrap()
    }
}

impl Serialize for CountryCode {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> StdResult<S::Ok, S::Error> {
        Serialize::serialize(self.0.as_str(), ser)
    }
}

impl<'a> Deserialize<'a> for CountryCode {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> StdResult<Self, D::Error> {
        <&str>::deserialize(des).map(Self::from)
    }
}

/// [`CountryCodes`] maps a country code with it's own country.
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCodes(BTreeMap<CountryCode, String>);

impl CountryCodes {
    /// `get` returns the currently defined codes.
    pub fn get() -> Result<CountryCodes> {
        // We expect the file to always be in the lib dir
        let scodes = include_str!("../data/iso3166.json");

        // We expect the file to be always well formatted
        serde_json::from_str(scodes).map_err(Error::JSONDeserialize)
    }

    /// `exists` check if a country code exists.
    pub fn exists(&self, code: &CountryCode) -> bool {
        self.0.contains_key(code)
    }

    /// `validate` validates a country code.
    pub fn validate(&self, code: &str) -> Result<()> {
        let c = CountryCode::new(code)?;

        if !self.exists(&c) {
            Err(Error::CodeNotFound)
        } else {
            Ok(())
        }
    }

    /// `find` finds a code from a country.
    pub fn find(&self, country: &str) -> Option<&CountryCode> {
        // Sadly, we have to iterate. TODO: improve
        for (k, v) in self.0.iter() {
            // TODO: improve
            if v.contains(country) {
                return Some(k);
            }
        }

        None
    }
}

mod test {
    #[allow(unused_imports)] // TODO
    use super::{CountryCodes, Result};

    #[test]
    fn is_valid() -> Result<()> {
        CountryCodes::get().map(|_| ())
    }

    #[test]
    fn validate() {
        const WRONG_CODES: &[&str] = &["abc", "AB", "13", "abcde", "XX", "ZZ"];
        const VALID_CODES: &[&str] = &["PN", "SM", "SV", "AD", "AX", "VU", "ZM"];

        let codes = CountryCodes::get().unwrap();

        for code in WRONG_CODES.iter() {
            assert!(codes.validate(code).is_err());
        }

        for code in VALID_CODES.iter() {
            assert!(codes.validate(code).is_ok());
        }
    }

    #[test]
    fn find() {
        const WRONG_COUNTRIES: &[&str] = &[
            "Soviet Union",
            "Sacred Roman Empire",
            "Aztec Empire",
            "Third Reich",
        ];
        const VALID_COUNTRIES: &[&str] =
            &["Kosovo", "United States", "Australia", "Vanuatu", "Zambia"];

        let codes = CountryCodes::get().unwrap();

        for country in WRONG_COUNTRIES.iter() {
            assert!(codes.find(country).is_none());
        }

        for country in VALID_COUNTRIES.iter() {
            assert!(codes.find(country).is_some());
        }
    }
}
