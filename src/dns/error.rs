#[cfg(feature = "result_error")]
mod error;
mod error_and_option;
#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "result_error")]
pub use error::Error;
#[cfg(feature = "logger")]
pub use logger::debug::{get_current_thread_logs, init_logger, logger_flush};

pub use error_and_option::ErrorAndOption;
