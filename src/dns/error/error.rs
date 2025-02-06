use std::fmt::{Debug, Display, Formatter};
use std::net::AddrParseError;

pub enum Error {
    AddrParseError(AddrParseError),
    NoServerAvailable,
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        Error::AddrParseError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddrParseError(e) => Display::fmt(&e, f),
            Error::NoServerAvailable => f.write_str("No server available"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddrParseError(e) => Debug::fmt(&e, f),
            Error::NoServerAvailable => f.write_str("No server available"),
        }
    }
}
