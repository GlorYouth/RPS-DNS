mod domain;
mod ip_addr;
mod soa;
mod txt;

pub use domain::{CNAME, NS};
pub use ip_addr::{A, AAAA};
pub use soa::SOA;
pub use txt::TXT;
