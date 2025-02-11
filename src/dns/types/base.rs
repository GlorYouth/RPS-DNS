mod dns_type;
mod domain;
#[cfg(feature = "fmt")]
mod fmt;
pub mod record;

pub use dns_type::{DnsType, DnsTypeNum};
pub use domain::RawDomain;
#[cfg(feature = "fmt")]
pub use fmt::{DnsClass, DnsTTL};
