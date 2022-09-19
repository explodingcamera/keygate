use crate::{Health, KeySignal};

pub trait Metadata: Send + Sync {
    fn version(&self) -> &'static str;
    fn health(&self) -> Health;
}

impl Metadata for KeySignal {
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn health(&self) -> Health {
        self.health
    }
}
