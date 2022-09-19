mod storage;

pub mod models;
pub mod traits;
use std::fmt::Debug;
use std::fmt::Display;

use models::Session;
pub use storage::SorageExt;
pub use storage::StorageType;
pub use storage::{InMemoryStorage, Storage};

#[derive(Clone, Copy)]
pub enum Health {
    Healthy,
    Starting,
    Unhealthy,
}

#[derive(Clone)]
pub struct Configuration {}

pub struct KeySignal {
    config: Configuration,
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

    fn get<T>(&self, key: &str) -> std::io::Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self.storage.get_u8(key);
        if bytes.is_empty() {
            return Ok(None);
        }

        rmp_serde::from_slice(bytes.as_slice())
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("failed to deserialize data: {e}"),
                )
            })
            .map(|v| Some(v))
    }

    fn set<T>(&self, key: &str, value: &T) -> std::io::Result<Option<()>>
    where
        T: serde::Serialize,
    {
        let val = rmp_serde::to_vec(value).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("failed to serialize data: {e}"),
            )
        })?;
        self.storage.set_u8(key, &val);
        Ok(Some(()))
    }
}
