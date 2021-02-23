//! Country codes as defined by ISO 3166-1 alpha-2.

use crate::error::Error;
use crate::result::Result;
use serde::{self, Deserialize, Serialize};
use std::char;
use std::collections::BTreeMap;

/// [`CODE_LENGTH`] is the length of an ISO 3166-1 alpha-2 code
const CODE_LENGTH: usize = 2;

/// [`Code`] is an ISO 3166-1 alpha-2 code
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Code(String);

impl Code {
    pub fn new(code: &str) -> Result<Code> {
        if !Code::is_valid(code) {
            return Err(Error::InvalidCode);
        }

        Ok(Code(code.into()))
    }

    /// `is_valid` returns if a provided code is valid
    /// by checking length and kind of chars used.
    pub fn is_valid(code: &str) -> bool {
        code.len() == CODE_LENGTH
            && code.find(|c: char| c.is_lowercase() || !c.is_ascii_alphabetic()) == None
    }

    /// `validate` validates the [`Code`]. Here only length and digites
    /// used are checked, not if the code is actually representative of a
    /// used code.
    pub fn validate(&self) -> Result<()> {
        if !Code::is_valid(&self.0) {
            return Err(Error::InvalidCode);
        }

        Ok(())
    }
}

/// [`Codes`] maps a country code with it's own country.
#[derive(Debug, Serialize, Deserialize)]
pub struct Codes(BTreeMap<Code, String>);

impl Codes {
    /// `get` returns the currently defined codes.
    pub fn get() -> Result<Codes> {
        // We expect the file to always be in the lib dir
        let scodes = include_str!("../data/codes.json");

        // We expect the file to be always well formatted
        serde_json::from_str(scodes)
            .map_err(Error::Deserialize)
    }

    /// `exists` check if a country code exists.
    pub fn exists(&self, code: &Code) -> bool {
        self.0.contains_key(code)
    }

    /// `validate` validates a country code.
    pub fn validate(&self, code: &str) -> Result<()> {
        let c = Code::new(code)?;

        if !self.exists(&c) {
            Err(Error::CodeNotFound)
        } else {
            Ok(())
        }
    }

    /// `find` finds a code from a country.
    pub fn find(&self, country: &str) -> Option<&Code> {
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
    use super::{Codes, Result};

    #[test]
    fn codes_is_valid() -> Result<()> {
        Codes::get().map(|_| ())
    }

    #[test]
    fn codes_validate() {
        const WRONG_CODES: &[&str] = &["abc", "AB", "13", "abcde", "XX", "ZZ"];
        const VALID_CODES: &[&str] = &["PN", "SM", "SV", "AD", "AX", "VU", "ZM"];

        let codes = Codes::get().unwrap();

        for code in WRONG_CODES.iter() {
            assert!(codes.validate(code).is_err());
        }

        for code in VALID_CODES.iter() {
            assert!(codes.validate(code).is_ok());
        }
    }

    #[test]
    fn codes_find() {
        const WRONG_COUNTRIES: &[&str] = &[
            "Soviet Union",
            "Sacred Roman Empire",
            "Aztec Empire",
            "Third Reich",
        ];
        const VALID_COUNTRIES: &[&str] =
            &["Kosovo", "United States", "Australia", "Vanuatu", "Zambia"];

        let codes = Codes::get().unwrap();

        for country in WRONG_COUNTRIES.iter() {
            assert!(codes.find(country).is_none());
        }

        for country in VALID_COUNTRIES.iter() {
            assert!(codes.find(country).is_some());
        }
    }
}
