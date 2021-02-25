//! Business Identifier Code (BIC, or SWIFT BIC or SWIFT code) as defined by ISO 9362.

use crate::iso3166::CountryCode;
use crate::iso9362::{BranchCode, InstitutionCode, LocationCode};
use serde::{Deserialize, Serialize};

/// [`BIC`] is an ISO 9362 BIC code.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BIC {
    pub institution: InstitutionCode,
    pub country: CountryCode,
    pub location: LocationCode,
    pub branch: BranchCode,
}
