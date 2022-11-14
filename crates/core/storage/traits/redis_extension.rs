use crate::storage::StorageError;
use crate::utils;
use redis::{aio::ConnectionManager, AsyncCommands};

#[async_trait::async_trait]
pub trait RedisExtensions {
    async fn get_and_deserialize<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned;
}

#[async_trait::async_trait]
impl RedisExtensions for ConnectionManager {
    async fn get_and_deserialize<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: Option<Vec<u8>> = self.clone().get(key).await?;
        match value {
            Some(bytes) => Ok(utils::serialize::from_bytes(&bytes)?),
            None => Ok(None),
        }
    }
}
