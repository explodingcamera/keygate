use std::sync::atomic::AtomicI64;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AtomicDateTime(AtomicI64);

impl AtomicDateTime {
    pub fn new() -> Self {
        Self(AtomicI64::new(0))
    }

    pub fn set(&self, date: DateTime<Utc>) {
        self.0.store(date.timestamp(), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get(&self) -> DateTime<Utc> {
        let time = self.0.load(std::sync::atomic::Ordering::Relaxed);
        let time = chrono::NaiveDateTime::from_timestamp_opt(time, 0).expect("Invalid timestamp, this is a bug");
        DateTime::from_utc(time, Utc)
    }
}
