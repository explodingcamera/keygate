use crate::storage::StorageError;
use redis::{aio::ConnectionManager, AsyncCommands};

#[async_trait::async_trait]
pub trait RedisExtensions {
    async fn get_deserialized<T>(&self, key: &str) -> Result<Option<T>, StorageError>;
}

#[async_trait::async_trait]
impl RedisExtensions for ConnectionManager {
    async fn get_deserialized<T>(&self, key: &str) -> Result<Option<T>, StorageError> {
        let value: Option<Vec<u8>> = self.clone().get(key).await?;
        match value {
            Some(bytes) => unimplemented!(),
            None => Ok(None),
        }
    }
}
