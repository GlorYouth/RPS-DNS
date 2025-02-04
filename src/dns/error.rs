mod logger;

#[cfg(debug_assertions)]
pub use logger::debug::init_logger;
