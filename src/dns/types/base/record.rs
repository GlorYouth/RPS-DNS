mod domain;
mod ip_addr;
mod soa;

pub use ip_addr::{A, AAAA};
pub use domain::{NS,CNAME};
pub use soa::SOA;
