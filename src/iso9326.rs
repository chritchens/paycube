//! Business Identifier Code (BIC or SWIFT BIC or SWIFT code) as defined by ISO 9362.

use crate::iso31661_alpha2::CountryCode;

pub struct IDCode(String);
pub struct LocationCode(String);
pub struct BranchCode(String);

/// [`BIC`] is an ISO 9362 BIC code.
pub struct BIC {
    pub id: IDCode,
    pub country: CountryCode,
    pub location: LocationCode,
    pub branch: BranchCode,
}
