use serde_json;
use std::error;
use std::fmt;

/// `Error` type of the crate.
#[derive(Debug)]
pub enum Error {
    InvalidCode,
    CodeNotFound,
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            Error::InvalidCode => "invalid code".into(),
            Error::CodeNotFound => "code not found".into(),
            Error::Serialize(source) => format!("{}", source),
            Error::Deserialize(source) => format!("{}", source),
            Error::Other(source) => source.into(),
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Serialize(ref source) => Some(source),
            Error::Deserialize(ref source) => Some(source),
            _ => None,
        }
    }
}
