//! IBAN's check digits, as defined by ISO 13616.

use arraystring::{typenum::U2, ArrayString};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CheckDigits(ArrayString<U2>);
