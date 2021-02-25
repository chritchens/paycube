//! Business Identifier Code (BIC or SWIFT BIC or SWIFT code) as defined by ISO 9362.

use crate::iso3166::CountryCode;
use arraystring::{
    typenum::{U2, U3, U4},
    ArrayString,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IDCode(ArrayString<U4>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocationCode(ArrayString<U2>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BranchCode(ArrayString<U3>);

/// [`BIC`] is an ISO 9362 BIC code.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BIC {
    pub id: IDCode,
    pub country: CountryCode,
    pub location: LocationCode,
    pub branch: BranchCode,
}
