//! IBAN's BBAN (_ _ Account Number), as defined by ISO 13616.

use arraystring::{typenum::U30, ArrayString};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BBAN(ArrayString<U30>);
