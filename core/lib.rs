pub mod models;
pub mod storage;
pub mod utils;

mod traits;
use storage::StorageError;
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
pub enum KeysignalError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("unknown error")]
    Unknown,
}

pub struct KeySignal {
    pub config: Configuration,
    storage: Box<dyn Storage + Send + Sync>,
    health: Health,
}

impl KeySignal {
    pub fn new(config: Configuration) -> KeySignal {
        let storage = InMemoryStorage::new();
        KeySignal::new_with_storage(config, Box::new(storage))
    }

    pub fn new_with_storage(
        config: Configuration,
        storage: Box<dyn Storage + Send + Sync>,
    ) -> KeySignal {
        KeySignal {
            config,
            storage,
            health: Health::Starting,
        }
    }
}
