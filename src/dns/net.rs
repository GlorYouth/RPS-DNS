mod query;

pub use query::NetQuery;
#[cfg(feature = "result_error")]
pub use query::NetQueryError;
