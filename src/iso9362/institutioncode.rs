//! BIC's (non-)financial institution code, as defined by ISO 9362.

use arraystring::{typenum::U4, ArrayString};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InstitutionCode(ArrayString<U4>);
