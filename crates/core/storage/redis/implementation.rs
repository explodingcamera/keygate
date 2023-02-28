use super::constants::*;

use crate::storage::{traits::*, LogicStorageError, StorageError};
use crate::utils::macros::{async_transaction, join_keys};
use crate::utils::validate::RefreshTokenError;
use crate::{config, models, utils, KeygateConfigInternal, Storage};

use super::utils::RedisExtensions;
use chrono::{DateTime, Utc};
use redis::{aio::ConnectionManager, AsyncCommands};

pub type RedisStorageError = redis::RedisError;

pub struct RedisStorage {
    pool: ConnectionManager,
    config: KeygateConfigInternal,
}

impl RedisStorage {
    pub async fn new(config: KeygateConfigInternal) -> Result<Self, StorageError> {
        let config::StorageOptions::Redis(storage_options) = config.read().map_err(StorageError::from)?.storage_options.clone() else {
            return Err(StorageError::Storage(LogicStorageError::NotFound(
                "no redis storage options".to_string(),
            )));
        };

        let redis_url = storage_options.redis_url.clone();

        let redis = redis::Client::open(redis_url.as_str())?;
        let pool = redis.get_tokio_connection_manager().await?;

        Ok(Self { pool, config })
    }

    pub fn pool(&self) -> ConnectionManager {
        self.pool.clone()
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: redis::FromRedisValue,
    {
        let value: Option<T> = self.pool().get(key).await?;
        Ok(value)
    }

    pub async fn get_deserialized<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.pool().get_deserialized(key).await
    }
}

impl StorageConfigExtension for RedisStorage {
    fn config(&self) -> &KeygateConfigInternal {
        &self.config
    }
}

impl Storage for RedisStorage {}

#[async_trait::async_trait]
impl StorageProcessExtension for RedisStorage {
    async fn process_by_id(&self, id: &str) -> Result<Option<models::Process>, StorageError> {
        self.get_deserialized::<models::Process>(&join_keys!(PROCESS_BY_ID, id))
            .await
    }

    async fn process_by_token(&self, token_id: &str) -> Result<Option<models::Process>, StorageError> {
        match self.get::<String>(&join_keys!(PROCESS_ID_BY_TOKEN, token_id)).await? {
            Some(id) => self.process_by_id(&id).await,
            None => Ok(None),
        }
    }

    async fn process_create(&self, process: &models::Process) -> Result<(), StorageError> {
        unimplemented!()
    }

    async fn process_update(&self, updated_process: &models::Process) -> Result<(), StorageError> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl StorageSessionExtension for RedisStorage {
    async fn session_create(
        &self,
        identity_id: &str,
        refresh_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError> {
        let Some (identity) = self.identity_by_id(identity_id).await? else {
            return Err(LogicStorageError::NotFound(format!("no identity with id {identity_id}")).into());
        };

        let (session, refresh_token) = utils::session::create_initial_session(identity_id, refresh_expires_at);

        let mut conn = self.pool();
        let sessions_key = join_keys!(IDENTITY_SESSIONS, identity_id);
        async_transaction!(&mut conn, &[sessions_key.clone()], {
            let mut sessions: Vec<String> = conn.get_deserialized(&sessions_key).await?.unwrap_or_default();
            sessions.push(session.id.clone());

            redis::pipe()
                .atomic()
                .set(
                    join_keys!(IDENTITY_SESSIONS, identity_id),
                    utils::encoding::to_bytes(&sessions)?,
                )
                .set(
                    join_keys!(SESSION_BY_ID, &session.id.clone()),
                    utils::encoding::to_bytes(&session)?,
                )
                .set(
                    join_keys!(REFRESH_TOKEN_BY_ID, &refresh_token.id.clone()),
                    utils::encoding::to_bytes(&refresh_token)?,
                )
                .query_async(&mut conn)
                .await?;

            Some(())
        });

        Ok((refresh_token, session))
    }

    async fn refresh_token_rotate(
        &self,
        refresh_token_id: &str,
        refresh_expires_at: DateTime<Utc>,
        access_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError> {
        let old_refresh_token_key = &join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id);

        let mut conn = self.pool();
        let (refresh_token, session): (models::RefreshToken, models::Session) =
            async_transaction!(&mut conn, &[old_refresh_token_key], {
                let refresh_token: models::RefreshToken = conn
                    .get_deserialized(&join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id))
                    .await?
                    .ok_or_else(|| {
                        LogicStorageError::NotFound(format!("refresh token with id {refresh_token_id} not found"))
                    })?;

                let session_id = refresh_token.session_id.clone();

                let session: models::Session = conn
                    .get_deserialized(&join_keys!(SESSION_BY_ID, &session_id))
                    .await?
                    .ok_or_else(|| LogicStorageError::NotFound(format!("session with id {session_id} not found")))?;

                match utils::validate::can_refresh(&refresh_token) {
                    Ok(_) => (),
                    Err(RefreshTokenError::ReuseError(e)) => {
                        self.session_reuse_detected(&refresh_token).await?;
                        return Err(RefreshTokenError::ReuseError(e).into());
                    }
                    Err(e) => return Err(e.into()),
                }

                if !utils::validate::can_refresh_session(&session) {
                    self.session_reuse_detected(&refresh_token).await?;
                    return Err(StorageError::Session("revoked".to_string()));
                }

                let res =
                    utils::session::rotate_refresh_token(refresh_token, session, refresh_expires_at, access_expires_at);

                let (new_refresh_token_id, old_refresh_token_id) =
                    (res.new_refresh_token.id.clone(), res.old_refresh_token.id.clone());

                redis::pipe()
                    .atomic()
                    .set(new_refresh_token_id, utils::encoding::to_bytes(&res.new_refresh_token)?)
                    .set(session_id, utils::encoding::to_bytes(&res.updated_session)?)
                    .set(old_refresh_token_id, utils::encoding::to_bytes(&res.old_refresh_token)?)
                    .query_async(&mut conn)
                    .await?;

                Some((res.new_refresh_token, res.updated_session))
            });

        Ok((refresh_token, session))
    }

    async fn access_token_revoke(&self, access_token_id: &str) -> Result<(), StorageError> {
        todo!()
    }

    async fn refresh_token_revoke(&self, refresh_token_id: &str) -> Result<(), StorageError> {
        todo!()
    }

    async fn session_reuse_detected(&self, refresh_token: &models::RefreshToken) -> Result<(), StorageError> {
        todo!()
    }

    async fn refresh_token_by_id(&self, refresh_token_id: &str) -> Result<Option<models::RefreshToken>, StorageError> {
        let refresh_token_bytes: Option<Vec<u8>> = self
            .pool()
            .get(&join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id))
            .await?;

        let Some(refresh_token_bytes) = refresh_token_bytes else {
            return Ok(None);
        };

        Ok(Some(utils::encoding::from_bytes(&refresh_token_bytes)?))
    }

    async fn session_by_id(&self, session_id: &str) -> Result<Option<models::Session>, StorageError> {
        self.get_deserialized::<models::Session>(&join_keys!(SESSION_BY_ID, session_id))
            .await
    }

    async fn sessions(&self, identity_id: &str) -> Result<Option<Vec<models::Session>>, StorageError> {
        self.get_deserialized::<Vec<models::Session>>(&join_keys!(IDENTITY_SESSIONS, identity_id))
            .await
    }

    async fn refresh_token_create(&self, refresh_token: &models::RefreshToken) -> Result<(), StorageError> {
        let refresh_token_bytes = utils::encoding::to_bytes(refresh_token)?;
        self.pool()
            .set(&join_keys!(REFRESH_TOKEN_BY_ID, &refresh_token.id), refresh_token_bytes)
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl StorageIdentityExtension for RedisStorage {
    async fn identity_create(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let identity_bytes = utils::encoding::to_bytes(identity)?;

        let identity_username = identity.username.clone();
        let identity_emails = identity
            .emails
            .iter()
            .map(|email| (join_keys!(IDENTITY_ID_BY_EMAIL, email.0.as_str()), identity.id.as_str()))
            .collect::<Vec<_>>();

        let mut conn = self.pool();
        let identity_key = join_keys!(IDENTITY_BY_ID, &identity.id);
        async_transaction!(&mut conn, &[identity_key.clone()], {
            if conn.exists(identity_key.clone()).await? {
                return Err(LogicStorageError::AlreadyExists("already exists".to_string()).into());
            }

            for (email_key, identity_id) in identity_emails.clone() {
                if conn.exists(email_key.clone()).await? {
                    return Err(LogicStorageError::AlreadyExists(format!("email already exists: {email_key}")).into());
                }
            }

            let mut pipe = &mut redis::pipe();
            pipe = pipe.atomic();

            pipe = pipe
                // Set the identity
                .set(join_keys!(IDENTITY_BY_ID, &identity.id), identity_bytes.clone())
                // Set the email index
                .set_multiple(&identity_emails);

            if let Some(username) = identity_username.clone() {
                // Set the username index
                pipe = pipe
                    .set(join_keys!(IDENTITY_ID_BY_USERNAME, &username), &identity.id)
                    // set the username secondary index (lexicographically sorted)
                    .zadd(IDENTITY_USERNAME_INDEX, username, 0)
            }

            pipe.query_async(&mut self.pool()).await?;

            Some(())
        });

        Ok(())
    }

    async fn identity_update(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let mut conn = self.pool();

        let identity_key = &join_keys!(IDENTITY_BY_ID, &identity.id);
        let identity_username = identity.username.clone();

        async_transaction!(&mut conn, &[identity_key], {
            let existing_identity: models::Identity = conn
                .get_deserialized(identity_key)
                .await?
                .ok_or_else(|| LogicStorageError::NotFound(format!("identity with id {identity_key} not found")))?;

            if &existing_identity == identity {
                return Ok(());
            }

            let existing_identity_username = existing_identity.username.clone();

            let mut pipe = &mut redis::pipe();
            pipe = pipe.atomic();

            if existing_identity.username.is_some()
                && (identity_username.is_none() || existing_identity_username != identity_username)
            {
                pipe = pipe
                    .del(join_keys!(
                        IDENTITY_ID_BY_USERNAME,
                        &existing_identity_username.clone().unwrap()
                    )) // remove the old username index
                    .zrem(IDENTITY_USERNAME_INDEX, existing_identity_username.clone().unwrap());
                // remove the old username from the index
            }

            if identity_username.is_some() && existing_identity_username != identity_username {
                pipe = pipe
                    .set(
                        join_keys!(IDENTITY_ID_BY_USERNAME, &identity_username.clone().unwrap()),
                        &identity.id,
                    ) // Set the new username index
                    .zadd(IDENTITY_USERNAME_INDEX, &identity_username.clone().unwrap(), 0);
                //  add the new username to the index
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
                        .map(|email| (join_keys!(IDENTITY_ID_BY_EMAIL, email.0.as_str()), identity.id.as_str()))
                        .collect::<Vec<_>>(),
                );
            }

            if let Some(existing_identity_username) = existing_identity_username {
                if identity_username.is_none() {
                    pipe = pipe.zrem(IDENTITY_USERNAME_INDEX, existing_identity_username);
                }
            }

            // Set the identity
            let x = pipe
                .set(
                    join_keys!(IDENTITY_BY_ID, &identity.id),
                    utils::encoding::to_bytes(identity)?,
                )
                .query_async(&mut self.pool())
                .await?;

            Some(())
        });

        Ok(())
    }

    async fn identity_by_username(&self, username: &str) -> Result<Option<models::Identity>, StorageError> {
        match self
            .get::<String>(&join_keys!(IDENTITY_ID_BY_USERNAME, username))
            .await?
        {
            Some(identity_id) => self.identity_by_id(&identity_id).await,
            None => Ok(None),
        }
    }

    async fn identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError> {
        match self.get::<String>(&join_keys!(IDENTITY_ID_BY_EMAIL, email)).await? {
            Some(identity_id) => self.identity_by_id(&identity_id).await,
            None => Ok(None),
        }
    }

    async fn identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        self.get_deserialized::<models::Identity>(&join_keys!(IDENTITY_BY_ID, id))
            .await
    }
}
