//! Country codes as defined by ISO 3166-1 alpha-2 (plus Kosovo's code, `XK`).

use crate::error::Error;
use crate::result::Result;
use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;

/// [`LENGTH`] is the length of an ISO 3166-1 alpha-2 code
const LENGTH: usize = 2;

/// [`Codes`] maps a country code with it's own country.
#[derive(Debug, Serialize, Deserialize)]
pub struct Codes(BTreeMap<String, String>);

impl Codes {
    /// `get` returns the currently defined codes.
    pub fn get() -> Result<Codes> {
        // We expect the file to always be in the lib dir
        let scodes = include_str!("../data/codes.json");

        // We expect the file to be always well formatted
        serde_json::from_str(scodes)
            .map(|res| res)
            .map_err(|err| Error::Deserialize(err))
    }

    /// `exists` checks if a country code exists.
    pub fn exists(&self, code: &str) -> bool {
        self.0.contains_key(code)
    }

    /// `validate` validates a country code.
    pub fn validate(&self, code: &str) -> Result<()> {
        if code.len() != LENGTH {
            return Err(Error::InvalidCode);
        }

        if !self.exists(code) {
            return Err(Error::CodeNotFound);
        }

        Ok(())
    }
}

mod test {
    #[warn(unused_imports)]
    use super::{Codes, Result};

    #[test]
    fn codes_exists() -> Result<()> {
        Codes::get().map(|_| ())
    }

    #[test]
    fn codes_validate() {
        const WRONG_CODES: &[&str] = &["abc", "AB", "13", "abcde", "XX", "ZZ"];
        const VALID_CODES: &[&str] = &["PN", "SM", "SV", "AD", "AX", "VU", "ZM"];

        let codes = Codes::get().unwrap();

        for code in WRONG_CODES.iter() {
            let res = codes.validate(code);
            assert!(res.is_err())
        }

        for code in VALID_CODES.iter() {
            let res = codes.validate(code);
            assert!(res.is_ok())
        }
    }
}
