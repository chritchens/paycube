//! BIC's branch code, as defined by ISO 9362.

use arraystring::{typenum::U3, ArrayString};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BranchCode(ArrayString<U3>);
