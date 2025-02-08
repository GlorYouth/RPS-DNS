mod error;
mod logger;

pub use error::Error;
#[allow(unused)]
#[cfg(feature = "logger")]
pub use logger::debug::{get_current_thread_logs, init_logger, logger_flush};
