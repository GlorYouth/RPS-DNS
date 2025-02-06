mod error;
mod logger;

pub use error::Error;
#[cfg(debug_assertions)]
pub use logger::debug::init_logger;
