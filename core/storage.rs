use downcast_rs::{impl_downcast, Downcast};
use thiserror::Error;

use self::{memory::InMemoryStorageError, redis::RedisStorageError, rocksdb::RocksDBStorageError};

pub mod constants;

mod memory;
pub type InMemoryStorage = memory::InMemoryStorage;

mod rocksdb;
pub type RocksDBStorage = rocksdb::RocksDBStorage;

mod redis;
pub type RedisStorage = redis::RedisStorage;

mod storage_serde_extension;
mod storage_utils_extension;
pub use storage_serde_extension::StorageSerdeExtension;
pub use storage_utils_extension::StorageUtilsExtension;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    InMemoryStorage(#[from] InMemoryStorageError),
    #[error(transparent)]
    RocksDBStorage(#[from] RocksDBStorageError),
    #[error(transparent)]
    RedisStorage(#[from] RedisStorageError),
    #[error("decoding error")]
    Decoding(#[from] rmp_serde::decode::Error),
    #[error("encoding error")]
    Encoding(#[from] rmp_serde::encode::Error),
    #[error(transparent)]
    Storage(#[from] LogicStorageError),
}

#[derive(Error, Debug)]
pub enum LogicStorageError {
    #[error("the key {0} already exists")]
    AlreadyExists(String),
    #[error("unknown storage error")]
    Unknown,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum StorageType {
    InMemory,
    RocksDB,
    Redis,
}

pub trait Storage: Downcast {
    /// Get a value from the storage, if it exists. If it doesn't exist, return None.
    /// Should be avoided if other methods (e.g get_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;

    /// Get a value from the storage, if it exists. If it doesn't exist, return None.
    /// Should be avoided if other methods (e.g get_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    fn _pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError>;

    /// Set a value in the storage. If the key already exists, overwrite it.
    /// Should be avoided if other methods (e.g set_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;

    /// Set a value in the storage. If the key already exists, overwrite it.
    /// Should be avoided if other methods (e.g set_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    fn _pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError>;

    /// Check if a key exists in the storage
    fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self._get_u8(key)?.is_some())
    }

    /// Check if a key exists in the storage
    fn pexists(&self, prefix: &str, key: &str) -> Result<bool, StorageError> {
        Ok(self._pget_u8(prefix, key)?.is_some())
    }
}

impl_downcast!(Storage);
