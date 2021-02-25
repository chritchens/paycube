//! BIC's location code, as defined by ISO 9362.

use arraystring::{typenum::U2, ArrayString};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocationCode(ArrayString<U2>);
