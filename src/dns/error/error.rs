use std::fmt::{Debug, Display, Formatter};
use crate::{AddrReaderError, DomainError};


pub enum Error {
    DomainError {
        source: Box<DomainError>,
    },

    AddrError {
        source: AddrReaderError,
    },

    InvalidVecLength { length: usize },
}

impl From<Box<DomainError>> for Error {
    fn from(source: Box<DomainError>) -> Error {
        Error::DomainError { source }
    }
}

impl From<AddrReaderError> for Error {
    fn from(source: AddrReaderError) -> Error {
        Error::AddrError { source }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}