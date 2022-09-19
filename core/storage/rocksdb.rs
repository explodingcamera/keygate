use std::io::{Error, ErrorKind, Result};

use rocksdb::{DBWithThreadMode, MultiThreaded};

use super::Storage;

type RocksDB = DBWithThreadMode<MultiThreaded>;

pub struct RocksDBStorage {
    db: RocksDB,
}

impl RocksDBStorage {
    pub fn new() -> Result<Self> {
        let opts = rocksdb::Options::default();
        let db =
            RocksDB::open(&opts, "./db").map_err(|_| Error::new(ErrorKind::Other, "oh no!"))?;
        Ok(Self { db })
    }
}

impl Storage for RocksDBStorage {
    fn get_u8(&self, key: &str) -> Vec<u8> {
        self.db.get(key).unwrap().unwrap()
    }

    fn set_u8(&self, key: &str, value: &[u8]) {
        self.db.put(key, value).unwrap()
    }
}
