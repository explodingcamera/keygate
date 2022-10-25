use rocksdb::{MultiThreaded, OptimisticTransactionDB};
use thiserror::Error;

use crate::{
    models::{self, Session},
    storage_constants::*,
    utils,
};

use super::{
    BaseStorage, LogicStorageError, Storage, StorageError, StorageIdentityExtension,
    StorageSerdeExtension,
};

#[derive(Error, Debug)]
pub enum RocksDBStorageError {
    #[error(transparent)]
    RocksDBError(#[from] rocksdb::Error),
    #[error("RocksDB error: {0}")]
    RocksDBStringError(String),
    #[error("unknown data store error")]
    Unknown,
}

type RocksDB = OptimisticTransactionDB<MultiThreaded>;

pub struct RocksDBStorage {
    db: RocksDB,
    session_cache: dashmap::DashMap<String, Session>,
}

impl RocksDBStorage {
    pub fn new() -> Result<Self, StorageError> {
        let opts = rocksdb::Options::default();

        let db = OptimisticTransactionDB::open(&opts, "./db")
            .map_err(|e| StorageError::RocksDBStorage(e.into()))?;

        Ok(Self {
            db,
            session_cache: dashmap::DashMap::new(),
        })
    }
}

impl Storage for RocksDBStorage {}

impl BaseStorage for RocksDBStorage {
    fn _get_u8(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let res = self.db.get(key).map_err(RocksDBStorageError::from)?;
        Ok(res)
    }

    fn _set_u8(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        self.db.put(key, value).map_err(RocksDBStorageError::from)?;
        Ok(())
    }

    fn _pget_u8(&self, prefix: &str, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self
            .db
            .get(join_keys!(prefix, key))
            .map_err(RocksDBStorageError::from)?)
    }

    fn _pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        self.db
            .put(join_keys!(prefix, key), value)
            .map_err(RocksDBStorageError::from)?;
        Ok(())
    }

    fn _del(&self, key: &str) -> Result<(), StorageError> {
        self.db.delete(key).map_err(RocksDBStorageError::from)?;
        Ok(())
    }

    fn _pdel(&self, prefix: &str, key: &str) -> Result<(), StorageError> {
        self.db
            .delete(join_keys!(prefix, key))
            .map_err(RocksDBStorageError::from)?;
        Ok(())
    }
}

impl StorageSerdeExtension for RocksDBStorage {}
impl StorageIdentityExtension for RocksDBStorage {
    fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let id = self._pget::<String>(IDENTITY_ID_BY_USERNAME, username)?;
        if let Some(id) = id {
            self.get_identity_by_id(id.as_str())
        } else {
            Ok(None)
        }
    }

    fn get_identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError> {
        let id = self._pget::<String>(IDENTITY_ID_BY_EMAIL, email)?;
        if let Some(id) = id {
            self.get_identity_by_id(id.as_str())
        } else {
            Ok(None)
        }
    }

    fn get_identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        self._pget::<models::Identity>(IDENTITY_BY_ID, id)
    }

    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        // Check if the username is already taken
        if self.exists(&join_keys!(IDENTITY_ID_BY_USERNAME, &identity.username))? {
            return Err(LogicStorageError::AlreadyExists("identity".to_string()).into());
        }

        // Check if the email is already taken
        if identity.emails.iter().any(|email| {
            self.exists(&join_keys!(IDENTITY_ID_BY_EMAIL, email.0))
                .unwrap_or(false)
        }) {
            return Err(LogicStorageError::AlreadyExists("identity".to_string()).into());
        }

        let tx = self.db.transaction();
        let username = &identity.username;

        // Set the username index
        tx.put(join_keys!(IDENTITY_ID_BY_USERNAME, username), &identity.id)
            .map_err(RocksDBStorageError::from)?;

        // Set the email index
        for email in &identity.emails {
            tx.put(join_keys!(IDENTITY_ID_BY_EMAIL, email.0), &identity.id)
                .map_err(RocksDBStorageError::from)?;
        }

        // Set the identity
        self._pset(IDENTITY_BY_ID, &identity.id, identity)?;

        Ok(())
    }

    fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        let existing_identity = self.get_identity_by_id(&identity.id)?;

        if let Some(existing_identity) = existing_identity {
            if identity == &existing_identity {
                return Ok(());
            }

            let tx = self.db.transaction();
            let username = &identity.username;
            let existing_username = &existing_identity.username;

            // emails have been updated
            if identity.emails != existing_identity.emails {
                for email in &existing_identity.emails {
                    tx.delete(join_keys!(IDENTITY_ID_BY_EMAIL, email.0))
                        .map_err(RocksDBStorageError::from)?;
                }
                for email in &identity.emails {
                    tx.put(join_keys!(IDENTITY_ID_BY_EMAIL, email.0), &identity.id)
                        .map_err(RocksDBStorageError::from)?;
                }
            }

            // username has been updated
            if username != existing_username {
                tx.delete(join_keys!(IDENTITY_ID_BY_USERNAME, existing_username))
                    .map_err(RocksDBStorageError::from)?;
                tx.put(join_keys!(IDENTITY_ID_BY_USERNAME, username), &identity.id)
                    .map_err(RocksDBStorageError::from)?;
            }

            // Set the identity
            let identity_bytes = utils::serialize::to_bytes(&identity)?;
            tx.put(join_keys!(IDENTITY_BY_ID, &identity.id), identity_bytes)
                .map_err(RocksDBStorageError::from)?;

            tx.commit().map_err(RocksDBStorageError::RocksDBError)?;
            Ok(())
        } else {
            Err(LogicStorageError::NotFound("identity".to_string()).into())
        }
    }
}
