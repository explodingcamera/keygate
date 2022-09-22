use r2d2::Pool;
use redis::{Client, Commands};
use thiserror::Error;

use crate::Storage;

use super::StorageError;

#[derive(Error, Debug)]
pub enum RedisStorageError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("connection error: {0}")]
    ConnectionError(#[from] r2d2::Error),
}

pub struct RedisStorage {
    pool: Pool<Client>,
}

impl Default for RedisStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl RedisStorage {
    pub fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let pool = r2d2::Pool::builder().max_size(15).build(client).unwrap();
        Self { pool }
    }

    fn get_pool(&self) -> Result<r2d2::PooledConnection<Client>, RedisStorageError> {
        self.pool.get().map_err(RedisStorageError::from)
    }

    pub fn hget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()?
            .hget(prefix, key)
            .map_err(RedisStorageError::from)?)
    }

    pub fn hset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .hset(prefix, key, value)
            .map_err(RedisStorageError::from)?)
    }
}

impl Storage for RedisStorage {
    fn get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self.get_pool()?.get(key).map_err(RedisStorageError::from)?)
    }

    fn set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .set(key, value)
            .map_err(RedisStorageError::from)?)
    }

    fn pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()?
            .get(prefix.to_owned() + ":" + key)
            .map_err(RedisStorageError::from)?)
    }

    fn pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .set(prefix.to_owned() + ":" + key, value)
            .map_err(RedisStorageError::from)?)
    }
}
