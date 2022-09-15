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
    fn get(&self, key: &str) -> Option<String> {
        self.db.get(key).unwrap().map(|_| "hello".to_string())
    }

    fn set(&self, key: &str, value: &str) {
        todo!()
    }
}
