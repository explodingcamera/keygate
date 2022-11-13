use super::{
    constants::*, BaseStorage, LogicStorageError, StorageError, StorageIdentityExtension,
    StorageProcessExtension, StorageSerdeExtension, StorageSessionExtension, StorageWithConfig,
};

use crate::{models, utils::serialize, KeygateConfigInternal, Storage};
use chrono::{DateTime, Utc};
use redis::{aio::ConnectionManager, AsyncCommands};

pub type RedisStorageError = redis::RedisError;

pub struct RedisStorage {
    pool: ConnectionManager,
    config: KeygateConfigInternal,
}

impl RedisStorage {
    pub async fn new(config: KeygateConfigInternal) -> Result<Self, StorageError> {
        let redis_url = config.read()?.storage_options.redis_url.clone();
        let redis = redis::Client::open(redis_url.as_str())?;
        let pool = redis.get_tokio_connection_manager().await?;

        Ok(Self { pool, config })
    }
}

impl StorageWithConfig for RedisStorage {
    fn get_config(&self) -> &KeygateConfigInternal {
        &self.config
    }
}

impl Storage for RedisStorage {}
impl StorageProcessExtension for RedisStorage {}

#[async_trait::async_trait]
impl StorageSessionExtension for RedisStorage {
    async fn add_session(&self, session: &models::Session) -> Result<(), StorageError> {
        todo!()
    }

    async fn refresh_token(
        &self,
        refresh_token_id: &str,
        refresh_expires_at: DateTime<Utc>,
        access_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError> {
        let old_refresh_token_key = &join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id);
        // let session_key = &join_keys!(SESSION_BY_ID, &session_id);

        todo!();

        // redis::transaction(&mut self.pool, &[old_refresh_token_key], |con, pipe| {
        //     let old_val: isize = con.get(key)?;
        // });
        // todo!();

        // let tx = self.db.transaction();

        // let refresh_token: models::RefreshToken = tx
        //     .get(&join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id))
        //     .map_err(RocksDBStorageError::from)?
        //     .ok_or_else(|| {
        //         LogicStorageError::NotFound(format!(
        //             "refresh token with id {} not found",
        //             refresh_token_id
        //         ))
        //     })
        //     .map(|t| utils::serialize::from_bytes(&t).map_err(StorageError::from))?
        //     .map_err(StorageError::from)?;

        // let session_id = refresh_token.session_id.clone();

        // let session: models::Session = tx
        //     .get(&join_keys!(SESSION_BY_ID, &session_id))
        //     .map_err(RocksDBStorageError::from)?
        //     .ok_or_else(|| {
        //         LogicStorageError::NotFound(format!("session with id {} not found", session_id))
        //     })
        //     .map(|t| utils::serialize::from_bytes(&t).map_err(StorageError::from))?
        //     .map_err(StorageError::from)?;

        // match utils::validate::can_refresh(&refresh_token) {
        //     Ok(_) => (),
        //     Err(RefreshTokenError::ReuseError(e)) => {
        //         self.reuse_detected(&refresh_token).await?;
        //         return Err(RefreshTokenError::ReuseError(e).into());
        //     }
        //     Err(e) => return Err(e.into()),
        // }

        // if !can_refresh_session(&session) {
        //     self.reuse_detected(&refresh_token).await?;
        //     return Err(StorageError::Session("revoked".to_string()));
        // }

        // let res = rotate_refresh_token(
        //     refresh_token,
        //     session,
        //     refresh_expires_at,
        //     access_expires_at,
        // );

        // tx.put(
        //     res.new_access_token.id.clone(),
        //     to_bytes(&res.new_access_token)?,
        // )
        // .map_err(RocksDBStorageError::from)?;

        // tx.put(
        //     res.new_refresh_token.id.clone(),
        //     to_bytes(&res.new_refresh_token)?,
        // )
        // .map_err(RocksDBStorageError::from)?;

        // tx.put(session_id, to_bytes(&res.updated_session)?)
        //     .map_err(RocksDBStorageError::from)?;

        // tx.put(refresh_token_id, to_bytes(&res.old_refresh_token)?)
        //     .map_err(RocksDBStorageError::from)?;

        // tx.commit().map_err(RocksDBStorageError::RocksDBError)?;

        // Ok((res.new_refresh_token, res.updated_session))
    }

    async fn revoke_access_token(&self, access_token_id: &str) -> Result<(), StorageError> {
        todo!()
    }

    async fn reuse_detected(
        &self,
        refresh_token: &models::RefreshToken,
    ) -> Result<(), StorageError> {
        todo!()
    }
}

#[async_trait::async_trait]
impl BaseStorage for RedisStorage {
    async fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .pool
            .clone()
            .get(key)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .pool
            .clone()
            .set(key, value)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self
            .pool
            .clone()
            .exists(key)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _del(&self, key: &str) -> Result<(), StorageError> {
        Ok(self
            .pool
            .clone()
            .del(key)
            .await
            .map_err(RedisStorageError::from)?)
    }

    async fn _create_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self
            .pool
            .clone()
            .set_nx(key, value)
            .await
            .map_err(RedisStorageError::from)?)
    }
}

impl StorageSerdeExtension for RedisStorage {}

#[async_trait::async_trait]
impl StorageIdentityExtension for RedisStorage {
    async fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
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
            .query_async(&mut self.pool.clone())
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
                .query_async(&mut self.pool.clone())
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
        let identity_id: Option<String> = self
            .pool
            .clone()
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
        let identity_id: Option<String> = self
            .pool
            .clone()
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
        let identity_bytes: Option<Vec<u8>> = self
            .pool
            .clone()
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
