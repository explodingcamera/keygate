use downcast_rs::{impl_downcast, Downcast};
use thiserror::Error;

use self::{redis::RedisStorageError, rocksdb::RocksDBStorageError};

pub mod constants;

mod rocksdb;
pub type RocksDBStorage = rocksdb::RocksDBStorage;

mod redis;
pub type RedisStorage = redis::RedisStorage;

pub mod traits;
pub use traits::*;

#[derive(Error, Debug)]
pub enum StorageError {
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
    #[error("paniced at {0}")]
    Panic(String),
}

#[derive(Error, Debug)]
pub enum LogicStorageError {
    #[error("the key {0} already exists")]
    AlreadyExists(String),
    #[error("the key {0} wasn't found")]
    NotFound(String),
    #[error("unknown storage error")]
    Unknown,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum StorageType {
    RocksDB,
    Redis,
}

#[async_trait::async_trait]
pub trait BaseStorage: Sync {
    /// Get a value from the storage, if it exists. If it doesn't exist, return None.
    /// Should be avoided if other methods (e.g get_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    async fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;

    /// Set a value in the storage. If the key already exists, overwrite it.
    /// Should be avoided if other methods (e.g set_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    async fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;

    /// Set a value in the storage. If the key already exists, return an error.
    /// Should be avoided if other methods (e.g set_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    async fn _create_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;

    /// Delete a value from the storage. If the key doesn't exist, return an error.
    /// Should be avoided if other methods (e.g delete_identity) are available, as these
    /// can have side effects (e.g. creating/updating an index or cache).
    async fn _del(&self, key: &str) -> Result<(), StorageError>;

    /// Check if a key exists in the storage
    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self._get_u8(key).await?.is_some())
    }
}

pub trait Storage:
    BaseStorage
    + traits::StorageIdentityExtension
    + traits::StorageProcessExtension
    + traits::StorageSessionExtension
    + Downcast
{
}

impl_downcast!(Storage);
