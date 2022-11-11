use crate::{models, utils::tokens, KeygateConfigInternal, KeygateError, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("unknown error")]
    Unknown,
}

pub struct Session {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Session {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Session {
    pub async fn validate(&self, session_token: &str) -> Result<models::Session, KeygateError> {
        todo!()
    }

    pub async fn invalidate(&self, session_token: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn all(&self, user_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn refresh_invalidate(&self, refresh_token: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn refresh(
        &self,
        refresh_token: &str,
    ) -> Result<(tokens::AccessToken, tokens::RefreshToken), KeygateError> {
        todo!()
    }
}
