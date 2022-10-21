use r2d2::Pool;
use redis::{Client, Commands};
use thiserror::Error;

use crate::{models, utils::serialize, Storage};

use super::{constants::IDENTITY_KEY, StorageError, StorageSerdeExtension, StorageUtilsExtension};

const REDIS_PREFIX: &str = "kg";

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

    fn get_key(&self, key: &str) -> String {
        format!("{REDIS_PREFIX}:{IDENTITY_KEY}:{key}")
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
    fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()?
            .get(REDIS_PREFIX.to_owned() + ":" + key)
            .map_err(RedisStorageError::from)?)
    }

    fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .set(REDIS_PREFIX.to_owned() + ":" + key, value)
            .map_err(RedisStorageError::from)?)
    }

    fn _pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()?
            .get(REDIS_PREFIX.to_owned() + ":" + prefix + ":" + key)
            .map_err(RedisStorageError::from)?)
    }

    fn _pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()?
            .set(REDIS_PREFIX.to_owned() + ":" + prefix + ":" + key, value)
            .map_err(RedisStorageError::from)?)
    }

    fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .get_pool()?
            .exists(REDIS_PREFIX.to_owned() + ":" + key)
            .map_err(RedisStorageError::from)?)
    }

    fn pexists(&self, prefix: &str, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .get_pool()?
            .exists(REDIS_PREFIX.to_owned() + ":" + prefix + ":" + key)
            .map_err(RedisStorageError::from)?)
    }
}

static REDIS_USERNAME_INDEX_KEY: &str = "zusernames";
static REDIS_IDENTITY_KEY: &str = "identities";
static REDIS_IDENTITY_BY_USERNAME_KEY: &str = "by_username";
static REDIS_IDENTITY_BY_EMAIL_KEY: &str = "by_email";

impl StorageSerdeExtension for RedisStorage {}
impl StorageUtilsExtension for RedisStorage {
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
            .hset(
                self.get_key(REDIS_IDENTITY_KEY),
                &identity.id,
                identity_bytes,
            )
            // Set the email index
            .hset_multiple(self.get_key(REDIS_IDENTITY_BY_EMAIL_KEY), &identity_emails)
            // Set the username index
            .hset(
                self.get_key(REDIS_IDENTITY_BY_USERNAME_KEY),
                &identity_username,
                &identity.id,
            )
            // set the username secondary index (lexicographically sorted)
            .zadd(REDIS_USERNAME_INDEX_KEY, identity_username, 0)
            .query(&mut *pool)
            .map_err(RedisStorageError::from)?;
        Ok(())
    }

    fn update_identity(&self, _identity: &models::Identity) -> Result<(), StorageError> {
        todo!()
    }
}
