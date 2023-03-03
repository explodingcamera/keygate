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
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Metadata {
    pub async fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub async fn status(&self) -> Health {
        Health::Healthy
    }
}
