use crate::{KeygateConfigInternal, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignupError {
    #[error("unknown error")]
    Unknown,
}

pub struct Signup {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Signup {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Signup {}
