use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Input Vec Len Mismatch, it should be {0} got {1}")]
    VecLenMismatch(usize, usize),

    #[error("Unknown Addr Type {0}")]
    UnknownAddrType(usize),

    #[error("Uncovered DNS Type {0}")]
    UncoveredDNSType(usize),

    #[error("DNS Question Number cannot be 0")]
    DNSQuestionNumber,
}

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.kind, f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.kind, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}
