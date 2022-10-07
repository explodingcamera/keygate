use super::{Storage, StorageError, StorageSerdeExtension, StorageUtilsExtension};
use dashmap::DashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InMemoryStorageError {}

#[derive(Default)]
pub struct InMemoryStorage {
    data: DashMap<String, Vec<u8>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for InMemoryStorage {
    fn get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self.data.get(key).map(|v| v.to_vec()))
    }

    fn set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        self.data.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .data
            .get(&(prefix.to_owned() + "-" + key))
            .map(|v| v.to_vec()))
    }

    fn pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        self.data
            .insert(prefix.to_owned() + "-" + key, value.to_vec());
        Ok(())
    }
}

impl StorageSerdeExtension for InMemoryStorage {}
impl StorageUtilsExtension for InMemoryStorage {}
