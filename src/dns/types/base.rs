mod dns_type;
mod domain;
#[cfg(feature = "fmt")]
mod fmt;
pub mod record;
mod string;

#[cfg(feature = "fmt")]
pub use dns_type::DnsType;
pub use dns_type::DnsTypeNum;
pub use domain::RawDomain;
#[cfg(feature = "fmt")]
pub use fmt::{DnsClass, DnsTTL};
pub use string::RawString;
