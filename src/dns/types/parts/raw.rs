mod response;
mod header;
mod question;
mod record;
mod request;
#[allow(unused)]
pub use response::RawResponse;
pub use header::RawResponseHeader;
pub use header::RawRequestHeader;
pub use question::RawQuestion;
pub use record::RawRecord;
pub use record::RecordDataType;
pub use request::RawRequest;
