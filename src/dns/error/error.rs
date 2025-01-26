use snafu::Snafu;
use crate::{AddrReaderError, DomainError};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{}", source))]
    DomainError {
        source: DomainError,
    },
    #[snafu(display("AddrReaderError: \n{}", source))]
    AddrError {
        source: AddrReaderError,
    },
    #[snafu(display("Invalid vec length: {}", length))]
    InvalidVecLength {
        length: usize,
    }
}