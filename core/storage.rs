mod memory;
pub type InMemoryStorage<'a> = memory::InMemoryStorage;

mod rocksdb;
pub type RocksDBStorage = rocksdb::RocksDBStorage;

#[derive(Clone, Copy)]
pub enum StorageType {
    InMemory,
}

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str);
}
