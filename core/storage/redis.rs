use super::{
    constants::*, BaseStorage, LogicStorageError, StorageError, StorageIdentityExtension,
    StorageSerdeExtension,
};
use crate::{models, utils::serialize, Storage};
use deadpool_redis::{Connection, Pool, PoolError};
use redis::AsyncCommands;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisStorageError {
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    Pool(#[from] PoolError),
    #[error(transparent)]
    CreatePool(#[from] deadpool_redis::CreatePoolError),
}

pub struct RedisStorage {
    pool: Pool,
}

impl RedisStorage {
    pub async fn new() -> Result<Self, StorageError> {
        let pool = deadpool_redis::Config::from_url("redis://127.0.0.1/")
            .create_pool(Some(deadpool::Runtime::Tokio1))
            .map_err(RedisStorageError::from)?;

        Ok(Self { pool })
    }

    async fn get_pool(&self) -> Result<Connection, RedisStorageError> {
        Ok(self.pool.get().await?)
    }

    // pub async fn hget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
    //     Ok(self
    //         .get_pool()
    //         .hget(prefix, key)
    //         .map_err(RedisStorageError::from)?)
    // }

    // pub async fn hset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
    //     Ok(self
    //         .get_pool()
    //         .hset(prefix, key, value)
    //         .map_err(RedisStorageError::from)?)
    // }
}

impl Storage for RedisStorage {}

#[async_trait::async_trait]
impl BaseStorage for RedisStorage {
    async fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()
            .await?
            .get(key)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()
            .await?
            .set(key, value)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .get_pool()
            .await?
            .get(join_keys!(prefix, key))
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .get_pool()
            .await?
            .set(join_keys!(prefix, key), value)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .get_pool()
            .await?
            .exists(key)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn pexists(&self, prefix: &str, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .get_pool()
            .await?
            .exists(join_keys!(prefix, key))
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _del(&self, key: &str) -> Result<(), StorageError> {
        Ok(self
            .get_pool()
            .await?
            .del(key)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _pdel(&self, prefix: &str, key: &str) -> Result<(), StorageError> {
        Ok(self
            .get_pool()
            .await?
            .del(join_keys!(prefix, key))
            .await
            .map_err(RedisStorageError::from)?)
    }
}

impl StorageSerdeExtension for RedisStorage {}

#[async_trait::async_trait]
impl StorageIdentityExtension for RedisStorage {
    async fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let mut pool = self.get_pool().await?;

        let identity_bytes = serialize::to_bytes(identity)?;

        let identity_username = identity.username.as_str();
        let identity_emails = identity
            .emails
            .iter()
            .map(|email| (email.0.as_str(), identity.id.as_str()))
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
            .query_async(&mut *pool)
            .await
            .map_err(RedisStorageError::from)?;

        Ok(())
    }

    async fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let identity_bytes = serialize::to_bytes(identity)?;
        let identity_username = identity.username.as_str();

        let existing_identity = self.get_identity_by_id(&identity.id).await?;

        if let Some(existing_identity) = existing_identity {
            if &existing_identity == identity {
                return Ok(());
            }

            let existing_identity_username = existing_identity.username.as_str();

            let mut pool = self.get_pool().await?;
            let mut pipe: &mut redis::Pipeline = &mut redis::pipe();
            pipe = pipe.atomic();

            if existing_identity_username != identity_username {
                pipe = pipe
                    .hdel(IDENTITY_ID_BY_USERNAME, existing_identity_username) // remove the old username index
                    .zrem(IDENTITY_USERNAME_INDEX, &existing_identity_username) // remove the old username from the index
                    .hset(IDENTITY_ID_BY_USERNAME, identity_username, &identity.id) // Set the new username index
                    .zadd(IDENTITY_USERNAME_INDEX, identity_username, 0); //  add the new username to the index
            }

            // check if the emails have changed
            if identity.emails != existing_identity.emails {
                // remove the old email index
                pipe = pipe.hdel(
                    IDENTITY_ID_BY_EMAIL,
                    &existing_identity
                        .emails
                        .iter()
                        .map(|email| email.0.as_str())
                        .collect::<Vec<_>>(),
                );
                // Set the new email index
                pipe = pipe.hset_multiple(
                    IDENTITY_ID_BY_EMAIL,
                    &identity
                        .emails
                        .iter()
                        .map(|email| (email.0.as_str(), identity.id.as_str()))
                        .collect::<Vec<_>>(),
                );
            }

            // Remove the old username index
            pipe = pipe.hdel(IDENTITY_ID_BY_USERNAME, &existing_identity_username);

            // Set the identity
            pipe.hset(IDENTITY_BY_ID, &identity.id, identity_bytes)
                .query_async(&mut *pool)
                .await
                .map_err(RedisStorageError::from)?;

            Ok(())
        } else {
            Err(LogicStorageError::NotFound("identity".to_string()).into())
        }
    }

    async fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let mut pool = self.get_pool().await?;

        let identity_id: Option<String> = pool
            .hget(IDENTITY_ID_BY_USERNAME, username)
            .await
            .map_err(RedisStorageError::from)?;

        if let Some(identity_id) = identity_id {
            self.get_identity_by_id(&identity_id).await
        } else {
            Ok(None)
        }
    }

    async fn get_identity_by_email(
        &self,
        email: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let mut pool = self.get_pool().await?;

        let identity_id: Option<String> = pool
            .hget(IDENTITY_ID_BY_EMAIL, email)
            .await
            .map_err(RedisStorageError::from)?;

        if let Some(identity_id) = identity_id {
            self.get_identity_by_id(&identity_id).await
        } else {
            Ok(None)
        }
    }

    async fn get_identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        let mut pool = self.get_pool().await?;
        let identity_bytes: Option<Vec<u8>> = pool
            .hget(IDENTITY_BY_ID, id)
            .await
            .map_err(RedisStorageError::from)?;

        if let Some(identity_bytes) = identity_bytes {
            let identity: models::Identity = serialize::from_bytes(&identity_bytes)?;

            Ok(Some(identity))
        } else {
            Ok(None)
        }
    }
}
