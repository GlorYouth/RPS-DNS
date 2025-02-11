mod domain;
mod ip_addr;
mod soa;

pub use domain::{CNAME, NS};
pub use ip_addr::{A, AAAA};
pub use soa::SOA;
