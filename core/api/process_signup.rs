use crate::{KeygateConfigInternal, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignupError {
    #[error("unknown error")]
    Unknown,

    #[error("invalid email")]
    InvalidEmail,

    #[error("invalid password")]
    InvalidPassword,

    #[error("invalid username")]
    InvalidUsername,

    #[error("this user already exists")]
    UserAlreadyExists,
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

impl Signup {
    pub async fn create_process_email(&self, email: String) -> Result<(), SignupError> {
        match self.storage.get_identity_by_email(&email).await {
            Ok(_) => Err(SignupError::Unknown),
            Err(_) => Ok(()),
        }
    }
}
