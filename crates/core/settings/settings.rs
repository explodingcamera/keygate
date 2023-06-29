use std::sync::{Arc, OnceLock};

use chrono::{DateTime, Utc};
use dashmap::DashMap;

use crate::KeygateInternal;

#[derive(Debug)]
pub struct KeygateSettings {
    keygate: OnceLock<Arc<KeygateInternal>>,
    data: DashMap<String, Setting>,
    updated_at: DateTime<Utc>,
}

impl KeygateSettings {
    pub fn new() -> Self {
        Self {
            keygate: OnceLock::new(),
            data: DashMap::new(),
            updated_at: DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp_micros(0).expect("invalid timestamp, this should never happen"),
                Utc,
            ),
        }
    }

    pub(crate) fn set_keygate(&self, keygate: Arc<KeygateInternal>) {
        self.keygate.set(keygate).unwrap();
    }
}

#[derive(Debug)]
enum Setting {
    String(String),
    Date(DateTime<Utc>),
    Integer(i64),
    Boolean(bool),
    Bytes(Vec<u8>),
}
