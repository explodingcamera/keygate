use super::Storage;
use dashmap::DashMap;

#[derive(Default)]
pub struct InMemoryStorage {
    data: DashMap<String, Vec<u8>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for InMemoryStorage {
    fn get_u8(&self, key: &str) -> Vec<u8> {
        self.data.get(key).unwrap().to_vec()
    }

    fn set_u8(&self, key: &str, value: &[u8]) {
        self.data.insert(key.to_string(), value.to_vec());
    }
}
