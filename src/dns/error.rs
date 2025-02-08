mod error;
mod logger;

pub use error::Error;
#[cfg(feature = "logger")]
#[allow(unused)]
pub use logger::debug::{get_current_thread_logs, init_logger, logger_flush};
