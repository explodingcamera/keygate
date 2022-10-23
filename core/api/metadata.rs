use crate::{Health, Keygate};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("unknown error")]
    Unknown,
}

pub trait Metadata: Send + Sync {
    fn version(&self) -> &'static str;
    fn health(&self) -> Health;
}

impl Metadata for Keygate {
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn health(&self) -> Health {
        self.health
    }
}
