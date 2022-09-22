mod storage;

pub mod models;
pub mod traits;
pub mod utils;
use std::fmt::Debug;

pub use storage::Storage;
use storage::StorageError;
pub use storage::StorageType;
pub use storage::{InMemoryStorage, RedisStorage, RocksDBStorage};

#[derive(Clone, Copy)]
pub enum Health {
    Healthy,
    Starting,
    Unhealthy,
}

#[derive(Clone, Debug)]
pub struct Configuration {}

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

    pub fn get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self.storage.get_u8(key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    pub fn set<T>(&self, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize,
    {
        let val = rmp_serde::to_vec(value)?;
        self.storage.set_u8(key, &val)
    }
}
