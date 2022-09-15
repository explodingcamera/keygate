use std::collections::HashMap;

use dashmap::DashMap;

use super::Storage;

#[derive(Default)]
pub struct InMemoryStorage {
    data: DashMap<String, String>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for InMemoryStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|s| s.to_string())
    }

    fn set(&self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}
