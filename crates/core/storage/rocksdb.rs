use chrono::{DateTime, Utc};
use rocksdb::{MultiThreaded, OptimisticTransactionDB};

use crate::{
    models::{self, Session},
    storage_constants::*,
    utils::{
        self,
        macros::join_keys,
        serialize::{from_bytes, to_bytes},
        session::{create_initial_session, rotate_refresh_token},
        validate::{can_refresh_session, RefreshTokenError},
    },
    KeygateConfigInternal,
};

use super::{
    BaseStorage, LogicStorageError, Storage, StorageError, StorageIdentityExtension,
    StorageProcessExtension, StorageSerdeExtension, StorageSessionExtension, StorageWithConfig,
};

pub type RocksDBStorageError = rocksdb::Error;
type RocksDB = OptimisticTransactionDB<MultiThreaded>;

pub struct RocksDBStorage {
    db: RocksDB,
    session_cache: dashmap::DashMap<String, Session>,
    config: KeygateConfigInternal,
}

impl RocksDBStorage {
    pub fn new(config: KeygateConfigInternal) -> Result<Self, StorageError> {
        let opts = rocksdb::Options::default();

        let db = OptimisticTransactionDB::open(&opts, "./db")?;

        Ok(Self {
            config,
            db,
            session_cache: dashmap::DashMap::new(),
        })
    }
}

impl StorageWithConfig for RocksDBStorage {
    fn get_config(&self) -> &KeygateConfigInternal {
        &self.config
    }
}

impl Storage for RocksDBStorage {}
impl StorageSerdeExtension for RocksDBStorage {}
impl StorageProcessExtension for RocksDBStorage {}

#[async_trait::async_trait]
impl StorageSessionExtension for RocksDBStorage {
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

        let tx = self.db.transaction();
        let sessions = tx
            .get(&join_keys!(IDENTITY_SESSIONS, &session.identity_id))?
            .unwrap_or_default();

        let mut sessions: Vec<String> = from_bytes(&sessions)?;
        sessions.push(session.id.clone());

        tx.put(
            &join_keys!(IDENTITY_SESSIONS, &session.identity_id),
            &to_bytes(&sessions)?,
        )?;

        tx.put(
            &join_keys!(SESSION_BY_ID, &session.id),
            &to_bytes(&session)?,
        )?;

        tx.put(
            &join_keys!(ACCESS_TOKEN_BY_ID, &access_token.id),
            &to_bytes(&access_token)?,
        )?;

        tx.put(
            &join_keys!(REFRESH_TOKEN_BY_ID, &refresh_token.id),
            &to_bytes(&refresh_token)?,
        )?;

        tx.commit()?;
        Ok((session, access_token, refresh_token))
    }

    async fn refresh_token(
        &self,
        refresh_token_id: &str,
        refresh_expires_at: DateTime<Utc>,
        access_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::AccessToken, models::Session), StorageError> {
        let tx = self.db.transaction();

        let refresh_token: models::RefreshToken = tx
            .get(&join_keys!(REFRESH_TOKEN_BY_ID, refresh_token_id))?
            .ok_or_else(|| {
                LogicStorageError::NotFound(format!(
                    "refresh token with id {} not found",
                    refresh_token_id
                ))
            })
            .map(|t| utils::serialize::from_bytes(&t))??;

        let session_id = refresh_token.session_id.clone();

        let session: models::Session = tx
            .get(&join_keys!(SESSION_BY_ID, &session_id))?
            .ok_or_else(|| {
                LogicStorageError::NotFound(format!("session with id {} not found", session_id))
            })
            .map(|t| utils::serialize::from_bytes(&t))??;

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

        tx.put(
            res.new_access_token.id.clone(),
            to_bytes(&res.new_access_token)?,
        )?;

        tx.put(
            res.new_refresh_token.id.clone(),
            to_bytes(&res.new_refresh_token)?,
        )?;

        tx.put(session_id, to_bytes(&res.updated_session)?)?;

        tx.put(refresh_token_id, to_bytes(&res.old_refresh_token)?)?;

        tx.commit()?;

        Ok((
            res.new_refresh_token,
            res.new_access_token,
            res.updated_session,
        ))
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
impl BaseStorage for RocksDBStorage {
    async fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let res = self.db.get(key)?;
        Ok(res)
    }

    async fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        self.db.put(key, value)?;
        Ok(())
    }

    async fn _del(&self, key: &str) -> Result<(), StorageError> {
        self.db.delete(key)?;
        Ok(())
    }

    async fn _create_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        if self.exists(key).await? {
            return Err(LogicStorageError::AlreadyExists(key.to_string()).into());
        }

        Ok(self.db.put(key, value)?)
    }
}

#[async_trait::async_trait]
impl StorageIdentityExtension for RocksDBStorage {
    async fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let id = self
            ._get::<String>(&join_keys!(IDENTITY_ID_BY_USERNAME, username))
            .await?;

        let Some(id) = id else {
            return Ok(None)
        };

        self.get_identity_by_id(id.as_str()).await
    }

    async fn get_identity_by_email(
        &self,
        email: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let id = self
            ._get::<String>(&join_keys!(IDENTITY_ID_BY_EMAIL, email))
            .await?;

        let Some(id) = id else {
            return Ok(None)
        };

        self.get_identity_by_id(id.as_str()).await
    }

    async fn get_identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        self._get::<models::Identity>(&join_keys!(IDENTITY_BY_ID, id))
            .await
    }

    async fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let tx = self.db.transaction();
        let username = &identity.username;

        if let Some(username) = username {
            // Check if the username is already taken
            if self
                .exists(&join_keys!(
                    IDENTITY_ID_BY_USERNAME,
                    &identity.username.clone().unwrap()
                ))
                .await?
            {
                return Err(LogicStorageError::AlreadyExists("identity".to_string()).into());
            }

            // Set the username index
            tx.put(join_keys!(IDENTITY_ID_BY_USERNAME, username), &identity.id)?;
        }

        // Check if the email is already taken
        for email in &identity.emails {
            if self
                .exists(&join_keys!(IDENTITY_ID_BY_EMAIL, email.0))
                .await?
            {
                return Err(LogicStorageError::AlreadyExists("identity".to_string()).into());
            }
        }

        // Set the email index
        for email in &identity.emails {
            tx.put(join_keys!(IDENTITY_ID_BY_EMAIL, email.0), &identity.id)?;
        }

        // Set the identity
        self._set(&join_keys!(IDENTITY_BY_ID, &identity.id), identity)
            .await?;

        Ok(())
    }

    async fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let existing_identity = self.get_identity_by_id(&identity.id).await?;

        let Some(existing_identity) = existing_identity else {
           return Err(LogicStorageError::NotFound("identity".to_string()).into())
        };

        if identity == &existing_identity {
            return Ok(());
        }

        let tx = self.db.transaction();
        let username = &identity.username;
        let existing_username = &existing_identity.username;

        // emails have been updated
        if identity.emails != existing_identity.emails {
            for email in &existing_identity.emails {
                tx.delete(join_keys!(IDENTITY_ID_BY_EMAIL, email.0))?;
            }
            for email in &identity.emails {
                tx.put(join_keys!(IDENTITY_ID_BY_EMAIL, email.0), &identity.id)?;
            }
        }

        // username has been updated
        if existing_username.is_some() && (username.is_none() || username != existing_username) {
            tx.delete(join_keys!(
                IDENTITY_ID_BY_USERNAME,
                &existing_username.clone().unwrap()
            ))?;
        }

        if username.is_some() && username != existing_username {
            tx.put(
                join_keys!(IDENTITY_ID_BY_USERNAME, &username.clone().unwrap()),
                &identity.id,
            )?;
        }

        // Set the identity
        let identity_bytes = utils::serialize::to_bytes(&identity)?;
        tx.put(join_keys!(IDENTITY_BY_ID, &identity.id), identity_bytes)?;

        tx.commit()?;
        Ok(())
    }
}
