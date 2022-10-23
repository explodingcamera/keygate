use crate::{Health, KeygateConfigInternal, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("unknown error")]
    Unknown,
}

pub struct Metadata {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Metadata {
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

pub trait MetadataTrait: Send + Sync {
    fn version(&self) -> &'static str;
    fn health(&self) -> Health;
}

impl MetadataTrait for Metadata {
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn health(&self) -> Health {
        Health::Healthy
    }
}
