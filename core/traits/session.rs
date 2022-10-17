use crate::{models, utils::tokens, Keygate, KeygateError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("unknown error")]
    Unknown,
}

pub trait Session: Send + Sync {
    fn session_validate(&self, session_token: &str) -> Result<models::Session, KeygateError>;

    fn session_invalidate(&self, session_token: &str) -> Result<(), KeygateError>;

    fn session_refresh_invalidate(&self, refresh_token: &str) -> Result<(), KeygateError>;

    fn session_refresh(
        &self,
        refresh_token: &str,
    ) -> Result<(tokens::SessionToken, tokens::RefreshToken), KeygateError>;

    fn sessions(&self, user_id: &str) -> Result<(), KeygateError>;
}

impl Session for Keygate {
    fn session_validate(&self, session_token: &str) -> Result<models::Session, KeygateError> {
        todo!()
    }

    fn session_invalidate(&self, session_token: &str) -> Result<(), KeygateError> {
        todo!()
    }

    fn sessions(&self, user_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    fn session_refresh_invalidate(&self, refresh_token: &str) -> Result<(), KeygateError> {
        todo!()
    }

    fn session_refresh(
        &self,
        refresh_token: &str,
    ) -> Result<(tokens::SessionToken, tokens::RefreshToken), KeygateError> {
        todo!()
    }
}
