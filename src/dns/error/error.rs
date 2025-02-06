use std::fmt::{Debug, Display, Formatter};
use std::net::AddrParseError;
use crate::dns::resolver::QueryError;

pub enum Error {
    AddrParseError(AddrParseError),
    QueryError(QueryError),
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        Error::AddrParseError(error)
    }
}

impl From<QueryError> for Error {
    fn from(error: QueryError) -> Self {
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
