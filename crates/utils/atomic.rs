use std::sync::atomic::AtomicI64;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct AtomicDateTime(AtomicI64);

impl AtomicDateTime {
    pub fn new() -> Self {
        Self(AtomicI64::new(0))
    }

    pub fn set(&self, date: OffsetDateTime) {
        self.0
            .store(date.unix_timestamp(), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get(&self) -> OffsetDateTime {
        let time = self.0.load(std::sync::atomic::Ordering::Relaxed);
        OffsetDateTime::from_unix_timestamp(time).expect("invalid internal date")
    }
}

impl Default for AtomicDateTime {
    fn default() -> Self {
        Self::new()
    }
}
