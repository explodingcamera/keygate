use r2d2::Pool;
use redis::{Client, Commands};

use crate::Storage;

pub struct RedisStorage {
    pool: Pool<Client>,
}

impl RedisStorage {
    pub fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let pool = r2d2::Pool::builder().max_size(15).build(client).unwrap();
        Self { pool }
    }
}

impl Storage for RedisStorage {
    fn get_u8(&self, key: &str) -> Vec<u8> {
        todo!()
    }

    fn set_u8(&self, key: &str, value: &[u8]) {
        todo!()
    }
}
