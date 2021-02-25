use std::error;
use std::fmt;

/// `Error` type of the crate.
#[derive(Debug)]
pub enum Error {
    InvalidCountryCode,
    CountryCodeNotFound,
    ParseCode(arraystring::Error),
    JSONSerialize(serde_json::Error),
    JSONDeserialize(serde_json::Error),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            Error::InvalidCountryCode => "invalid country code".into(),
            Error::CountryCodeNotFound => "country code not found".into(),
            Error::ParseCode(source) => format!("code parsing error: {}", source),
            Error::JSONSerialize(source) => format!("json serialization error: {}", source),
            Error::JSONDeserialize(source) => format!("json deserialization error: {}", source),
            Error::Other(source) => source.into(),
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ParseCode(ref source) => Some(source),
            Error::JSONSerialize(ref source) => Some(source),
            Error::JSONDeserialize(ref source) => Some(source),
            _ => None,
        }
    }
}
