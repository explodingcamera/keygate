mod memory;
pub type InMemoryStorage = memory::InMemoryStorage;

mod rocksdb;
pub type RocksDBStorage = rocksdb::RocksDBStorage;

mod redis;
pub type RedisStorage = redis::RedisStorage;

#[derive(Clone, Copy)]
pub enum StorageType {
    InMemory,
}

pub trait Storage {
    fn get_u8(&self, key: &str) -> Vec<u8>;
    fn set_u8(&self, key: &str, value: &[u8]);
}

pub struct SorageExt();
impl SorageExt {
    // has to be a boxed reference, we can't use a trait object here since it's not sized
    #[allow(clippy::borrowed_box)]
    pub fn get<T>(storage: &Box<dyn Storage + Send + Sync>, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = storage.get_u8(key);
        if bytes.is_empty() {
            return None;
        }
        rmp_serde::from_slice(bytes.as_slice()).ok()
    }

    // has to be a boxed reference, we can't use a trait object here since it's not sized
    #[allow(clippy::borrowed_box)]
    pub fn set<T>(storage: &Box<dyn Storage + Send + Sync>, key: &str, value: &T) -> Option<()>
    where
        T: serde::Serialize,
    {
        let val = rmp_serde::to_vec(value).unwrap();
        storage.set_u8(key, &val);
        Some(())
    }
}
