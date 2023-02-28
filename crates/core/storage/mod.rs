pub mod errors;
pub use errors::*;

pub mod traits;
pub use traits::*;

// storage backends
mod backends;
pub use backends::*;

mod redis;
mod sql;
pub type SQLStorage = sql::SQLStorage;
pub type RedisStorage = redis::RedisStorage;
