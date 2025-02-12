#[cfg(feature = "result_error")]
mod error;
mod result_and_error;
#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "result_error")]
pub use error::NetError;
#[cfg(feature = "logger")]
pub use logger::debug::{get_current_thread_logs, init_logger, logger_flush, set_println_enabled};

pub use result_and_error::ResultAndError;
#[cfg(feature = "result_error")]
pub use result_and_error::error_trait as error_trait;
