#[cfg(feature = "result_error")]
mod error;
#[cfg(feature = "logger")]
mod logger;
mod result_and_error;

#[cfg(feature = "result_error")]
pub use error::NetError;
#[cfg(feature = "logger")]
pub use logger::debug::{get_current_thread_logs, init_logger, logger_flush, set_println_enabled};

#[cfg(feature = "result_error")]
pub use error::ErrorFormat;
pub use result_and_error::ResultAndError;
pub use result_and_error::Wrapper;
