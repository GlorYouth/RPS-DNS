mod dns_type;
mod domain;
#[cfg(feature = "fmt")]
mod fmt;
pub mod record;

pub use dns_type::*;
pub use domain::*;
#[cfg(feature = "fmt")]
pub use fmt::*;
