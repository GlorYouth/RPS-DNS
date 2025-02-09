mod header;
mod others;
mod question;
mod record;
mod request;
mod response;

#[cfg(feature = "fmt")]
pub use others::DnsClass;
#[cfg(feature = "fmt")]
pub use others::DnsTTL;
pub use record::RecordDataType;
#[cfg(feature = "fmt")]
pub use record::RecordFmtType;
pub use request::Request;
pub use response::Response;
