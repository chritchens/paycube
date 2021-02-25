//! International Bank Account Number (IBAN) as defined by ISO 13616.

use crate::iso3166::CountryCode;
use arraystring::{
    typenum::{U2, U30},
    ArrayString,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CheckDigits(ArrayString<U2>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BBAN(ArrayString<U30>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IBAN {
    pub country: CountryCode,
    pub check: CheckDigits,
    pub bban: BBAN,
}
