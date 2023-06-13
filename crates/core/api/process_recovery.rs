use crate::{KeygateConfigInternal, KeygateError, KeygateSql};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecoveryError {
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug)]
pub struct Recovery {
    config: KeygateConfigInternal,
    storage: KeygateSql,
}

impl Recovery {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateSql) -> Self {
        Self { config, storage }
    }
}

impl Recovery {
    pub async fn init(&self) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn complete(&self) -> Result<(), KeygateError> {
        todo!()
    }
}
