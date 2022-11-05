use crate::{
    config,
    models::{self, BaseProcess, UsernameEmailSignupProcess},
    utils, KeygateConfigInternal, KeygateStorage,
};
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

    fn get_config(&self) -> Result<config::Configuration, SignupError> {
        Ok(self
            .config
            .read()
            .map_err(|_| SignupError::Unknown)?
            .clone())
    }
}

impl Signup {
    pub async fn init_signup_process(
        &self,
        username: Option<String>,
        email: Option<String>,
        device_id: String,
    ) -> Result<BaseProcess<UsernameEmailSignupProcess>, SignupError> {
        let config = self.get_config()?;

        if !utils::validate::is_valid_device_id(&device_id) {
            return Err(SignupError::Unknown);
        }

        if config.identity.signup_with_email {
            if let Some(email) = email.clone() {
                match self.storage.get_identity_by_email(&email).await {
                    Err(_) => return Err(SignupError::Unknown),
                    Ok(Some(user)) => return Err(SignupError::UserAlreadyExists),
                    Ok(None) => {}
                };
            } else if config.identity.signup_require_email {
                return Err(SignupError::InvalidEmail);
            }
        }

        if config.identity.signup_require_username {
            if let Some(username) = username.clone() {
                match self.storage.get_identity_by_username(&username).await {
                    Err(_) => return Err(SignupError::Unknown),
                    Ok(Some(user)) => return Err(SignupError::UserAlreadyExists),
                    Ok(None) => {}
                };
            } else if config.identity.signup_require_username {
                return Err(SignupError::InvalidUsername);
            }
        }

        let process = models::BaseProcess {
            process: models::UsernameEmailSignupProcess {
                device_id,
                username,
                email,
            },
            id: utils::random::secure_random_id(),
            created_at: chrono::Utc::now().timestamp().unsigned_abs(),
            expires_at: chrono::Utc::now()
                .timestamp()
                .unsigned_abs()
                .checked_add(config.identity.signup_process_lifetime)
                .ok_or(SignupError::Unknown)?,
        };

        self.storage
            .create_process(&models::Process::UsernameEmailSignup(process.clone()))
            .await
            .map_err(|_| SignupError::Unknown)?;

        Ok(process)
    }

    pub async fn init_oidc_signup_process(&self, email: String) -> Result<(), SignupError> {
        unimplemented!()
    }
}
