mod answer;
mod header;
mod question;
mod record;
mod request;
#[allow(unused)]
pub use answer::RawAnswer;
pub use header::RawAnswerHeader;
pub use header::RawRequestHeader;
pub use question::RawQuestion;
pub use record::RawRecord;
pub use record::RecordDataType;
pub use request::RawRequest;
