mod header;
mod question;
mod record;
mod request;
mod response;
pub use header::RawRequestHeader;
pub use header::RawResponseHeader;
pub use question::RawQuestion;
pub use record::RawRecord;
pub use record::RecordDataType;
pub use request::RawRequest;
#[allow(unused)]
pub use response::RawResponse;
