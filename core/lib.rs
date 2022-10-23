use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

mod api;
pub mod models;
pub mod utils;

pub mod config;
use api::identity::IdentityError;
use config::Configuration;
pub use config::Configuration as KeygateConfig;

mod storage;
use storage::StorageError;

pub use storage::traits;
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

#[derive(Error, Debug)]
pub enum KeygateError {
    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Identity(#[from] IdentityError),

    #[error("unknown error")]
    Unknown,
}

pub type KeygateResult<T> = Result<T, KeygateError>;
type KeygateConfigInternal = Arc<RwLock<Configuration>>;
type KeygateStorage = Arc<dyn Storage + Send + Sync>;

pub struct Keygate {
    pub config: KeygateConfigInternal,
    pub storage: KeygateStorage,
    pub health: Arc<RwLock<Health>>,

    pub identity: api::identity::Identity,
}

impl Keygate {
    pub fn new(config: Configuration) -> Result<Keygate, KeygateError> {
        let res = match config.storage_type {
            StorageType::InMemory => {
                Keygate::new_with_storage(config, Arc::new(InMemoryStorage::new()))
            }
            StorageType::RocksDB => match RocksDBStorage::new() {
                Ok(storage) => Keygate::new_with_storage(config, Arc::new(storage)),
                Err(e) => return Err(e.into()),
            },
            StorageType::Redis => match RedisStorage::new() {
                Ok(storage) => Keygate::new_with_storage(config, Arc::new(storage)),
                Err(e) => return Err(e.into()),
            },
        };

        Ok(res)
    }

    pub fn new_with_storage(
        config: Configuration,
        storage: Arc<dyn Storage + Send + Sync>,
    ) -> Keygate {
        let config = Arc::new(RwLock::new(config));

        Keygate {
            config: config.clone(),
            storage: storage.clone(),
            health: Arc::new(RwLock::new(Health::Starting)),
            identity: api::identity::Identity::new(config, storage),
        }
    }
}
