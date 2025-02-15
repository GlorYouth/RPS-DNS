pub mod bench_func;
mod dns;

pub use dns::resolver as resolver;
pub use dns::error as error;


pub use paste::paste as paste;