pub mod models;
pub mod storage;
pub mod utils;

mod traits;
use storage::StorageError;
use storage::StorageSerdeExtension;
pub use traits::all::*;

use std::fmt::Debug;
pub use storage::Storage;
pub use storage::StorageType;
pub use storage::{InMemoryStorage, RedisStorage, RocksDBStorage};
use thiserror::Error;

#[derive(Clone, Copy)]
pub enum Health {
    Healthy,
    Starting,
    Unhealthy,
}

#[derive(Clone, Debug)]
pub struct Configuration {}

#[derive(Error, Debug)]
pub enum KeygateError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("unknown error")]
    Unknown,
}

pub struct Keygate {
    pub config: Configuration,
    storage: Box<dyn Storage + Send + Sync>,
    health: Health,
}

impl Keygate {
    pub fn new(config: Configuration) -> Keygate {
        let storage = InMemoryStorage::new();
        Keygate::new_with_storage(config, Box::new(storage))
    }

    pub fn new_with_storage(
        config: Configuration,
        storage: Box<dyn Storage + Send + Sync>,
    ) -> Keygate {
        Keygate {
            config,
            storage,
            health: Health::Starting,
        }
    }
}
