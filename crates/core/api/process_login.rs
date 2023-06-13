use crate::{
    models::{self},
    KeygateConfigInternal, KeygateError, KeygateSql,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("unknown error")]
    Unknown,

    #[error("invalid device id")]
    InvalidDeviceId,

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

#[derive(Debug)]
pub struct Login {
    config: KeygateConfigInternal,
    storage: KeygateSql,
}

impl Login {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateSql) -> Self {
        Self { config, storage }
    }
}

impl Login {
    pub async fn init_login_process(
        &self,
        username_or_email: &str,
        device_id: &str,
    ) -> Result<models::Process, KeygateError> {
        todo!();

        // if !utils::validate::is_valid_id(device_id) {
        //     return Err(LoginError::InvalidDeviceId.into());
        // }

        // let config = self.get_config()?;
        // let username_or_email = username_or_email.trim().to_string();
        // let is_email = username_or_email.contains('@');

        // let identity = if is_email {
        //     if !config.identity.login_with_email {
        //         return Err(LoginError::InvalidEmail.into());
        //     }

        //     if !utils::validate::is_valid_email(&username_or_email) {
        //         return Err(LoginError::InvalidEmail.into());
        //     }

        //     self.storage
        //         .identity_by_email(&username_or_email)
        //         .await
        //         .map_err(|_| LoginError::Unknown)?
        // } else {
        //     if !config.identity.login_with_username {
        //         return Err(LoginError::InvalidUsername.into());
        //     }

        //     if !utils::validate::is_valid_username(&username_or_email) {
        //         return Err(LoginError::InvalidUsername.into());
        //     }

        //     self.storage
        //         .identity_by_username(&username_or_email)
        //         .await
        //         .map_err(|_| LoginError::Unknown)?
        // };

        // let identity = match identity {
        //     Some(identity) => identity,
        //     None => return Err(LoginError::Unknown.into()),
        // };

        // let data = UsernameEmailLoginProcess {
        //     device_id: device_id.to_string(),
        //     identity_id: identity.id,
        // };

        // let process = models::Process {
        //     completed_at: None,
        //     id: utils::random::secure_random_id(),
        //     data: Some(models::process::Data::UsernameEmailLogin(data)),
        //     created_at: chrono::Utc::now().timestamp(),
        //     expires_at: chrono::Utc::now()
        //         .timestamp()
        //         .checked_add(config.identity.login_process_lifetime)
        //         .ok_or(LoginError::Unknown)?,
        // };

        // self.storage
        //     .process_create(&process)
        //     .await
        //     .map_err(|_| LoginError::Unknown)?;

        // Ok(process)
    }

    pub async fn get_login_process(
        &self,
        device_id: &str,
        email_process_id: &str,
    ) -> Result<models::Process, KeygateError> {
        todo!();

        // if !utils::validate::is_valid_id(device_id) {
        //     return Err(KeygateError::ValidationError("invalid device id".to_string()));
        // }

        // if !utils::validate::is_valid_id(email_process_id) {
        //     return Err(KeygateError::ValidationError("invalid process id".to_string()));
        // }

        // let process = self
        //     .storage
        //     .process_by_id(email_process_id)
        //     .await
        //     .map_err(|_| KeygateError::Unknown)?
        //     .ok_or(KeygateError::Unknown)?;

        // let data = match process.data.clone() {
        //     Some(models::process::Data::UsernameEmailLogin(process)) => process,
        //     _ => return Err(KeygateError::Unknown),
        // };

        // if let Some(models::process::Data::UsernameEmailLogin(process)) = process.data.clone() {
        //     if process.device_id != device_id {
        //         return Err(KeygateError::ValidationError("invalid device id".to_string()));
        //     }
        // } else {
        //     return Err(KeygateError::Unknown);
        // }

        // Ok(process)
    }

    pub async fn validate_password(&self, password: &str, identity_id: &str) -> Result<(), KeygateError> {
        todo!();
        // let config = self.get_config()?;

        // if !utils::validate::is_valid_id(identity_id) {
        //     return Err(LoginError::Unknown.into());
        // }

        // if !utils::validate::is_valid_password(password) {
        //     return Err(LoginError::InvalidPassword.into());
        // }

        // let identity = self
        //     .storage
        //     .identity_by_id(identity_id)
        //     .await
        //     .map_err(|_| LoginError::Unknown)?
        //     .ok_or(LoginError::Unknown)?;

        // let Some(internal) = &identity.internal else {
        //     return Err(LoginError::Unknown.into());
        // };

        // let Some(password_hash) = &internal.password_hash else {
        //     return Err(LoginError::NoPassword.into());
        // };

        // if !hash::verify(password, password_hash).map_err(|_| LoginError::WrongPassword)? {
        //     return Err(LoginError::InvalidPassword.into());
        // }

        // if config.identity.password_min_length > 0 && password.len() < config.identity.password_min_length {
        //     return Err(LoginError::InvalidPassword.into());
        // }

        // Ok(())
    }
}
