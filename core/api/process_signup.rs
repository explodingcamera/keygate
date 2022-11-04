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
        username_or_email: String,
        device_id: String,
    ) -> Result<BaseProcess<UsernameEmailSignupProcess>, SignupError> {
        if !utils::validate::is_valid_device_id(&device_id) {
            return Err(SignupError::Unknown);
        }

        let config = self.get_config()?;

        if config.identity.enable_usernames {
            match self.storage.get_identity_by_email(&username_or_email).await {
                Err(_) => return Err(SignupError::Unknown),
                Ok(Some(user)) => return Err(SignupError::UserAlreadyExists),
                Ok(None) => {}
            };
        }

        let is_email = if username_or_email.contains('@') {
            if !username_or_email.contains('.') {
                return Err(SignupError::InvalidEmail);
            }

            if config.identity.enable_emails {
                match self.storage.get_identity_by_email(&username_or_email).await {
                    Err(_) => return Err(SignupError::Unknown),
                    Ok(Some(user)) => return Err(SignupError::UserAlreadyExists),
                    Ok(None) => {}
                };
            }
            true
        } else {
            if username_or_email.len() < 3 {
                return Err(SignupError::InvalidUsername);
            }
            false
        };

        let (username, email) = match is_email {
            true => (None, Some(username_or_email)),
            false => (Some(username_or_email), None),
        };

        let process = models::BaseProcess {
            process: models::UsernameEmailSignupProcess {
                device_id,
                username,
                email,
            },
            id: utils::random::secure_random_id(),
            created_at: chrono::Utc::now().timestamp().unsigned_abs(),
            expires_at: chrono::Utc::now().timestamp().unsigned_abs()
                + config.identity.signup_process_lifetime,
        };

        self.storage
            .create_process(&models::Processs::UsernameEmailSignup(process.clone()))
            .await
            .map_err(|_| SignupError::Unknown)?;

        Ok(process)
    }

    pub async fn init_oidc_signup_process(&self, email: String) -> Result<(), SignupError> {
        unimplemented!()
    }
}
