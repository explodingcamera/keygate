use crate::{
    config,
    models::{self, BaseProcess, UsernameEmailLoginProcess},
    utils::{self, hash},
    KeygateConfigInternal, KeygateError, KeygateStorage,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("unknown error")]
    Unknown,

    #[error("invalid email")]
    InvalidEmail,

    #[error("invalid password")]
    InvalidPassword,

    #[error("invalid username")]
    InvalidUsername,

    #[error("no password set")]
    NoPassword,

    #[error("wrong password")]
    WrongPassword,
}

pub struct Login {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Login {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }

    fn get_config(&self) -> Result<config::Configuration, LoginError> {
        Ok(self.config.read().map_err(|_| LoginError::Unknown)?.clone())
    }
}

impl Login {
    pub async fn init_login_process(
        &self,
        username_or_email: &str,
        device_id: &str,
    ) -> Result<BaseProcess<UsernameEmailLoginProcess>, KeygateError> {
        if !utils::validate::is_valid_device_id(device_id) {
            return Err(LoginError::Unknown.into());
        }

        let config = self.get_config()?;
        let username_or_email = username_or_email.trim().to_string();
        let is_email = username_or_email.contains('@');

        let identity = if is_email {
            if !config.identity.login_with_email {
                return Err(LoginError::InvalidEmail.into());
            }

            if !utils::validate::is_valid_email(&username_or_email) {
                return Err(LoginError::InvalidEmail.into());
            }

            self.storage
                .get_identity_by_email(&username_or_email)
                .await
                .map_err(|_| LoginError::Unknown)?
        } else {
            if !config.identity.login_with_username {
                return Err(LoginError::InvalidUsername.into());
            }

            if !utils::validate::is_valid_username(&username_or_email) {
                return Err(LoginError::InvalidUsername.into());
            }

            self.storage
                .get_identity_by_username(&username_or_email)
                .await
                .map_err(|_| LoginError::Unknown)?
        };

        let identity = match identity {
            Some(identity) => identity,
            None => return Err(LoginError::Unknown.into()),
        };

        let process = UsernameEmailLoginProcess {
            identity_id: identity.id,
            device_id: device_id.to_string(),
        };

        let process = BaseProcess {
            completed_at: None,
            id: utils::random::secure_random_id(),
            process,
            created_at: chrono::Utc::now().timestamp(),
            expires_at: chrono::Utc::now()
                .timestamp()
                .checked_add(config.identity.login_process_lifetime)
                .ok_or(LoginError::Unknown)?,
        };

        self.storage
            .create_process(&models::Process::UsernameEmailLogin(process.clone()))
            .await
            .map_err(|_| LoginError::Unknown)?;

        Ok(process)
    }

    pub async fn get_login_process(
        &self,
        device_id: &str,
        email_process_id: &str,
    ) -> Result<BaseProcess<UsernameEmailLoginProcess>, KeygateError> {
        if !utils::validate::is_valid_device_id(device_id) {
            return Err(KeygateError::ValidationError(
                "invalid device id".to_string(),
            ));
        }

        if !utils::validate::is_valid_id(email_process_id) {
            return Err(KeygateError::ValidationError(
                "invalid process id".to_string(),
            ));
        }

        let process = self
            .storage
            .process_by_id(email_process_id)
            .await
            .map_err(|_| KeygateError::Unknown)?;

        let process = match process {
            Some(models::Process::UsernameEmailLogin(process)) => process,
            _ => return Err(KeygateError::Unknown),
        };

        if process.process.device_id != device_id {
            return Err(KeygateError::ValidationError(
                "invalid device id".to_string(),
            ));
        }

        Ok(process)
    }

    pub fn validate_password(
        &self,
        password: &str,
        identity: &models::Identity,
    ) -> Result<(), LoginError> {
        let config = self.get_config()?;

        if !utils::validate::is_valid_password(password) {
            return Err(LoginError::InvalidPassword);
        }

        let Some(password_hash) = &identity.password_hash else {
            return Err(LoginError::NoPassword);
        };

        if !hash::verify(password, password_hash).map_err(|_| LoginError::WrongPassword)? {
            return Err(LoginError::InvalidPassword);
        }

        if config.identity.password_min_length > 0
            && password.len() < config.identity.password_min_length
        {
            return Err(LoginError::InvalidPassword);
        }

        Ok(())
    }
}
