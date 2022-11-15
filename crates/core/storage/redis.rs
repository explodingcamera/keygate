use super::{
    constants::*, BaseStorage, LogicStorageError, RedisExtensions, StorageError,
    StorageIdentityExtension, StorageProcessExtension, StorageSerdeExtension,
    StorageSessionExtension, StorageWithConfig,
};

use crate::utils::{
    self,
    macros::{async_transaction, join_keys},
    serialize::{self, to_bytes},
    session::{create_initial_session, rotate_refresh_token},
    validate::{can_refresh_session, RefreshTokenError},
};
use crate::{models, KeygateConfigInternal, Storage};
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

    pub fn pool(&self) -> ConnectionManager {
        self.pool.clone()
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
    async fn create_session(
        &self,
        identity_id: &str,
        refresh_expires_at: DateTime<Utc>,
        access_expires_at: DateTime<Utc>,
    ) -> Result<(models::Session, models::AccessToken, models::RefreshToken), StorageError> {
        let Some (identity) = self.get_identity_by_id(identity_id).await? else {
            return Err(LogicStorageError::NotFound(format!("no identity with id {identity_id}")).into());
        };

        let (session, access_token, refresh_token) =
            create_initial_session(identity_id, refresh_expires_at, access_expires_at);

        let mut conn = self.pool();
        let sessions_key = join_keys!(IDENTITY_SESSIONS, identity_id);
        async_transaction!(&mut conn, &[sessions_key.clone()], {
            let mut sessions: Vec<String> = conn
                .get_and_deserialize(&sessions_key)
                .await?
                .unwrap_or_default();
            sessions.push(session.id.clone());

            redis::pipe()
                .atomic()
                .set(
                    join_keys!(IDENTITY_SESSIONS, identity_id),
                    to_bytes(&sessions)?,
                )
                .set(
                    join_keys!(SESSION_BY_ID, &session.id.clone()),
                    to_bytes(&session)?,
                )
                .set(
                    join_keys!(ACCESS_TOKEN_BY_ID, &access_token.id.clone()),
                    to_bytes(&access_token)?,
                )
                .set(
                    join_keys!(REFRESH_TOKEN_BY_ID, &refresh_token.id.clone()),
                    to_bytes(&refresh_token)?,
                )
                .query_async(&mut conn)
                .await?;

            Some(())
        });

        Ok((session, access_token, refresh_token))
    }

    async fn refresh_token(
        &self,
        refresh_token_id: &str,
        refresh_expires_at: DateTime<Utc>,
        access_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::AccessToken, models::Session), StorageError> {
        let old_refresh_token_key = &join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id);

        let mut conn = self.pool();
        let (refresh_token, access_token, session): (
            models::RefreshToken,
            models::AccessToken,
            models::Session,
        ) = async_transaction!(&mut conn, &[old_refresh_token_key], {
            let refresh_token: models::RefreshToken = conn
                .get_and_deserialize(&join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id))
                .await?
                .ok_or_else(|| {
                    LogicStorageError::NotFound(format!(
                        "refresh token with id {refresh_token_id} not found"
                    ))
                })?;

            let session_id = refresh_token.session_id.clone();

            let session: models::Session = conn
                .get_and_deserialize(&join_keys!(SESSION_BY_ID, &session_id))
                .await?
                .ok_or_else(|| {
                    LogicStorageError::NotFound(format!("session with id {session_id} not found"))
                })?;

            match utils::validate::can_refresh(&refresh_token) {
                Ok(_) => (),
                Err(RefreshTokenError::ReuseError(e)) => {
                    self.reuse_detected(&refresh_token).await?;
                    return Err(RefreshTokenError::ReuseError(e).into());
                }
                Err(e) => return Err(e.into()),
            }

            if !can_refresh_session(&session) {
                self.reuse_detected(&refresh_token).await?;
                return Err(StorageError::Session("revoked".to_string()));
            }

            let res = rotate_refresh_token(
                refresh_token,
                session,
                refresh_expires_at,
                access_expires_at,
            );

            let (new_access_token_id, new_refresh_token_id, old_refresh_token_id) = (
                res.new_access_token.id.clone(),
                res.new_refresh_token.id.clone(),
                res.old_refresh_token.id.clone(),
            );

            redis::pipe()
                .atomic()
                .set(new_access_token_id, to_bytes(&res.new_access_token)?)
                .set(new_refresh_token_id, to_bytes(&res.new_refresh_token)?)
                .set(session_id, to_bytes(&res.updated_session)?)
                .set(old_refresh_token_id, to_bytes(&res.old_refresh_token)?)
                .query_async(&mut conn)
                .await?;

            Some((
                res.new_refresh_token,
                res.new_access_token,
                res.updated_session,
            ))
        });

        Ok((refresh_token, access_token, session))
    }

    async fn revoke_access_token(&self, access_token_id: &str) -> Result<(), StorageError> {
        todo!()
    }

    async fn revoke_refresh_token(&self, refresh_token_id: &str) -> Result<(), StorageError> {
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
        Ok(self.pool().get(key).await?)
    }

    async fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self.pool().set(key, value).await?)
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        Ok(self.pool().exists(key).await?)
    }

    async fn _del(&self, key: &str) -> Result<(), StorageError> {
        Ok(self.pool().del(key).await?)
    }

    async fn _create_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        Ok(self.pool().set_nx(key, value).await?)
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
            .map(|email| {
                (
                    join_keys!(IDENTITY_ID_BY_EMAIL, email.0.as_str()),
                    identity.id.as_str(),
                )
            })
            .collect::<Vec<_>>();

        let mut conn = self.pool();
        let identity_key = join_keys!(IDENTITY_BY_ID, &identity.id);
        async_transaction!(&mut conn, &[identity_key.clone()], {
            if conn.exists(identity_key.clone()).await? {
                return Err(LogicStorageError::AlreadyExists("already exists".to_string()).into());
            }

            for (email_key, identity_id) in identity_emails.clone() {
                if conn.exists(email_key.clone()).await? {
                    return Err(LogicStorageError::AlreadyExists(format!(
                        "email already exists: {email_key}"
                    ))
                    .into());
                }
            }

            redis::pipe()
                .atomic()
                // Set the identity
                .set(
                    join_keys!(IDENTITY_BY_ID, &identity.id),
                    identity_bytes.clone(),
                )
                // Set the email index
                .set_multiple(&identity_emails)
                // Set the username index
                .set(
                    join_keys!(IDENTITY_ID_BY_USERNAME, identity_username),
                    &identity.id,
                )
                // set the username secondary index (lexicographically sorted)
                .zadd(IDENTITY_USERNAME_INDEX, identity_username, 0)
                .query_async(&mut self.pool())
                .await?;

            Some(())
        });

        Ok(())
    }

    async fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let mut conn = self.pool();

        let identity_key = &join_keys!(IDENTITY_BY_ID, &identity.id);
        let identity_username = identity.username.as_str();

        async_transaction!(&mut conn, &[identity_key], {
            let existing_identity: models::Identity = conn
                .get_and_deserialize(identity_key)
                .await?
                .ok_or_else(|| {
                    LogicStorageError::NotFound(format!(
                        "identity with id {identity_key} not found"
                    ))
                })?;

            if &existing_identity == identity {
                return Ok(());
            }

            let existing_identity_username = existing_identity.username.as_str();
            let mut pipe = &mut redis::pipe();
            pipe = pipe.atomic();

            if existing_identity_username != identity_username {
                pipe = pipe
                    .del(join_keys!(
                        IDENTITY_ID_BY_USERNAME,
                        existing_identity_username
                    )) // remove the old username index
                    .zrem(IDENTITY_USERNAME_INDEX, existing_identity_username) // remove the old username from the index
                    .set(
                        join_keys!(IDENTITY_ID_BY_USERNAME, identity_username),
                        &identity.id,
                    ) // Set the new username index
                    .zadd(IDENTITY_USERNAME_INDEX, identity_username, 0); //  add the new username to the index
            }

            // check if the emails have changed
            if identity.emails != existing_identity.emails {
                // remove the old email index
                pipe = pipe.del(
                    &existing_identity
                        .emails
                        .iter()
                        .map(|email| join_keys!(IDENTITY_ID_BY_EMAIL, email.0.as_str()))
                        .collect::<Vec<_>>(),
                );
                // Set the new email index
                pipe = pipe.set_multiple(
                    &identity
                        .emails
                        .iter()
                        .map(|email| {
                            (
                                join_keys!(IDENTITY_ID_BY_EMAIL, email.0.as_str()),
                                identity.id.as_str(),
                            )
                        })
                        .collect::<Vec<_>>(),
                );
            }

            // Remove the old username index
            pipe = pipe.del(join_keys!(
                IDENTITY_ID_BY_USERNAME,
                existing_identity_username
            ));

            // Set the identity
            let x = pipe
                .set(
                    join_keys!(IDENTITY_BY_ID, &identity.id),
                    serialize::to_bytes(identity)?,
                )
                .query_async(&mut self.pool())
                .await?;

            Some(())
        });

        Ok(())
    }

    async fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let identity_id: Option<String> = self
            .pool()
            .get(&join_keys!(IDENTITY_ID_BY_USERNAME, username))
            .await?;

        let Some(identity_id) = identity_id else {
            return Ok(None)
        };

        self.get_identity_by_id(&identity_id).await
    }

    async fn get_identity_by_email(
        &self,
        email: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let identity_id: Option<String> = self
            .pool()
            .get(&join_keys!(IDENTITY_ID_BY_EMAIL, email))
            .await?;

        let Some(identity_id) = identity_id else {
            return Ok(None)
        };

        self.get_identity_by_id(&identity_id).await
    }

    async fn get_identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        let identity_bytes: Option<Vec<u8>> =
            self.pool().get(&join_keys!(IDENTITY_BY_ID, id)).await?;

        let Some(identity_bytes) = identity_bytes else {
            return Ok(None);
        };

        let identity: models::Identity = serialize::from_bytes(&identity_bytes)?;
        Ok(Some(identity))
    }
}
