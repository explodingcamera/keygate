mod storage;

pub mod traits;
use std::sync::Arc;

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

        KeySignal {
            config,
            storage: Box::new(storage),
            health: Health::Starting,
        }
    }

    pub fn set(&self, val: &str) {
        self.storage.set("test", val);
    }

    pub fn get(&self) -> Option<String> {
        self.storage.get("test")
    }
}
