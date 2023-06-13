use crate::{Health, KeygateConfigInternal, KeygateSql};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug)]
pub struct Metadata {
    config: KeygateConfigInternal,
    storage: KeygateSql,
}

impl Metadata {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateSql) -> Self {
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
