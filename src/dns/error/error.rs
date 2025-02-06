use crate::dns::net::NetQueryError;
use std::fmt::{Debug, Display, Formatter};
use std::net::AddrParseError;

pub enum Error {
    AddrParseError(AddrParseError),
    QueryError(NetQueryError),
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        Error::AddrParseError(error)
    }
}

impl From<NetQueryError> for Error {
    fn from(error: NetQueryError) -> Self {
        Error::QueryError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddrParseError(e) => Display::fmt(&e, f),
            Error::QueryError(e) => Display::fmt(&e, f),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddrParseError(e) => Debug::fmt(&e, f),
            Error::QueryError(e) => Debug::fmt(&e, f),
        }
    }
}
