use rocksdb::{DBWithThreadMode, MultiThreaded};
use thiserror::Error;

use crate::models::{self, Session};

use super::{Storage, StorageError, StorageSerdeExtension, StorageUtilsExtension};

#[derive(Error, Debug)]
pub enum RocksDBStorageError {
    #[error("RocksDB error: {0}")]
    RocksDBError(#[from] rocksdb::Error),
    #[error("unknown data store error")]
    Unknown,
}

type RocksDB = DBWithThreadMode<MultiThreaded>;

pub struct RocksDBStorage {
    db: RocksDB,
    session_cache: dashmap::DashMap<String, Session>,
}

impl RocksDBStorage {
    pub fn new() -> Result<Self, RocksDBStorageError> {
        let opts = rocksdb::Options::default();
        let db = RocksDB::open(&opts, "./db")?;

        Ok(Self {
            db,
            session_cache: dashmap::DashMap::new(),
        })
    }
}

impl Storage for RocksDBStorage {
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
            .get(prefix.to_owned() + ":" + key)
            .map_err(RocksDBStorageError::from)?)
    }

    fn _pset_u8(&self, prefix: &str, key: &str, value: &[u8]) -> Result<(), StorageError> {
        self.db
            .put(prefix.to_owned() + ":" + key, value)
            .map_err(RocksDBStorageError::from)?;
        Ok(())
    }
}

impl StorageSerdeExtension for RocksDBStorage {}
impl StorageUtilsExtension for RocksDBStorage {
    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        self._get::<models::Identity>(":")?;
        Ok(())
    }
}
