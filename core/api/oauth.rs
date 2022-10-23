use crate::{KeygateConfigInternal, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("unknown error")]
    Unknown,
}

pub struct OAuth {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl OAuth {
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

pub trait OAuthTrait: Send + Sync {}

impl OAuthTrait for OAuth {}
