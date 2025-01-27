use crate::{AddrReaderError, DomainError};
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{}", source))]
    DomainError {
        #[snafu(source(from(DomainError, Box::new)))]
        source: Box<DomainError>,
    },
    #[snafu(display("AddrReaderError: \n{}", source))]
    AddrError {
        #[snafu(source(from(AddrReaderError, Box::new)))]
        source: Box<AddrReaderError>,
    },
    #[snafu(display("Invalid vec length: {}", length))]
    InvalidVecLength { length: usize },
}
