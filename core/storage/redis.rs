use super::{
    constants::*, BaseStorage, StorageError, StorageIdentityExtension, StorageSerdeExtension,
};
use crate::{models, utils::serialize, Storage};
use r2d2::Pool;
use redis::{Client, Commands};
use thiserror::Error;

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

impl RedisStorage {
    pub fn new() -> Result<Self, StorageError> {
        let client = redis::Client::open("redis://127.0.0.1/").map_err(RedisStorageError::from)?;
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(client)
            .map_err(RedisStorageError::from)?;
        Ok(Self { pool })
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

impl Storage for RedisStorage {}
impl BaseStorage for RedisStorage {
    fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self.get_pool()?.get(key).map_err(RedisStorageError::from)?)
    }

    fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .set(key, value)
            .map_err(RedisStorageError::from)?)
    }

    fn _pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()?
            .get(join_keys!(prefix, key))
            .map_err(RedisStorageError::from)?)
    }

    fn _pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .set(join_keys!(prefix, key), value)
            .map_err(RedisStorageError::from)?)
    }

    fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .get_pool()?
            .exists(key)
            .map_err(RedisStorageError::from)?)
    }

    fn pexists(&self, prefix: &str, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .get_pool()?
            .exists(prefix.to_owned() + ":" + key)
            .map_err(RedisStorageError::from)?)
    }
}

impl StorageSerdeExtension for RedisStorage {}
impl StorageIdentityExtension for RedisStorage {
    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let mut pool = self.get_pool()?;

        let identity_bytes = serialize::to_bytes(identity)?;

        let identity_username = identity.username.as_str();
        let identity_emails = identity
            .emails
            .iter()
            .map(|email| (email.email.as_str(), identity.id.as_str()))
            .collect::<Vec<_>>();

        redis::pipe()
            .atomic()
            // Set the identity
            .hset(IDENTITY_BY_ID, &identity.id, identity_bytes)
            // Set the email index
            .hset_multiple(IDENTITY_ID_BY_EMAIL, &identity_emails)
            // Set the username index
            .hset(IDENTITY_ID_BY_USERNAME, &identity_username, &identity.id)
            // set the username secondary index (lexicographically sorted)
            .zadd(IDENTITY_USERNAME_INDEX, identity_username, 0)
            .query(&mut *pool)
            .map_err(RedisStorageError::from)?;

        Ok(())
    }

    fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let identity_bytes = serialize::to_bytes(identity)?;
        let identity_username = identity.username.as_str();
        let identity_emails = identity
            .emails
            .iter()
            .map(|email| (email.email.as_str(), identity.id.as_str()))
            .collect::<Vec<_>>();

        todo!()
    }

    fn get_identity(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        todo!()
    }

    fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        todo!()
    }

    fn get_identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError> {
        todo!()
    }

    fn get_identity_by_usern(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        todo!()
    }
}
