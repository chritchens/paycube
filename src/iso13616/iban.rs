//! International Bank Account Number (IBAN) as defined by ISO 13616.

use crate::iso13616::{CheckDigits, BBAN};
use crate::iso3166::CountryCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IBAN {
    pub country: CountryCode,
    pub check: CheckDigits,
    pub bban: BBAN,
}
