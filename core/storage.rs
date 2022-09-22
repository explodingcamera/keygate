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
    InMemoryStorage(#[from] InMemoryStorageError),
    #[error(transparent)]
    RocksDBStorage(#[from] RocksDBStorageError),
    #[error(transparent)]
    RedisStorage(#[from] RedisStorageError),
    #[error("decoding error")]
    Decoding(#[from] rmp_serde::decode::Error),
    #[error("encoding error")]
    Encoding(#[from] rmp_serde::encode::Error),
}

#[derive(Clone, Copy)]
pub enum StorageType {
    InMemory,
}

pub trait Storage: Downcast {
    fn get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;
    fn pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError>;
}

impl_downcast!(Storage);
