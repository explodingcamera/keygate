use crate::{models::EmailLoginProcess, KeygateConfigInternal, KeygateError, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("unknown error")]
    Unknown,
}

pub struct Login {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Login {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Login {
    pub async fn init_email_process(
        &self,
        device_id: &str,
    ) -> Result<EmailLoginProcess, KeygateError> {
        todo!()
    }

    pub async fn get_email_process(
        &self,
        device_id: &str,
        email_process_id: &str,
    ) -> Result<EmailLoginProcess, KeygateError> {
        todo!()
    }

    // pub fn login_submit_password(
    //     &self,
    //     login_process_id: &str,
    //     password: &str,
    // ) -> Result<LoginProcess, KeygateError> {
    //     todo!()
    // }

    // pub fn login_submit_magic_link(
    //     &self,
    //     login_process_id: &str,
    //     password: &str,
    // ) -> Result<LoginProcess, KeygateError> {
    //     todo!()
    // }

    // pub fn login_submit_otp(
    //     &self,
    //     login_process_id: &str,
    //     device_id: &str,
    //     magic_link: &str,
    // ) -> Result<LoginProcess, KeygateError> {
    //     todo!()
    // }
}
