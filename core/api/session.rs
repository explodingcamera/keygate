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
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Session {
    pub fn session_validate(&self, session_token: &str) -> Result<models::Session, KeygateError> {
        todo!()
    }

    pub fn session_invalidate(&self, session_token: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub fn sessions(&self, user_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub fn session_refresh_invalidate(&self, refresh_token: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub fn session_refresh(
        &self,
        refresh_token: &str,
    ) -> Result<(tokens::SessionToken, tokens::RefreshToken), KeygateError> {
        todo!()
    }
}
