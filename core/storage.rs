use downcast_rs::{impl_downcast, Downcast};
use thiserror::Error;

use self::{memory::InMemoryStorageError, redis::RedisStorageError, rocksdb::RocksDBStorageError};

mod memory;
pub type InMemoryStorage = memory::InMemoryStorage;

mod rocksdb;
pub type RocksDBStorage = rocksdb::RocksDBStorage;

mod redis;
pub type RedisStorage = redis::RedisStorage;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    InMemoryStorageError(#[from] InMemoryStorageError),
    #[error(transparent)]
    RocksDBStorageError(#[from] RocksDBStorageError),
    #[error(transparent)]
    RedisStorageError(#[from] RedisStorageError),
    #[error("decoding error")]
    DecodingError(#[from] rmp_serde::decode::Error),
    #[error("encoding error")]
    EncodingError(#[from] rmp_serde::encode::Error),
}

#[derive(Clone, Copy)]
pub enum StorageType {
    InMemory,
}

pub trait Storage: Downcast {
    fn get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn get_prefix_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;
    fn set_prefix_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError>;
}

impl_downcast!(Storage);
