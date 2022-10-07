use crate::{models::LoginFlow, Keygate, KeygateError};
static PREFIX: &str = "loginflow";

pub trait Login: Send + Sync {
    fn login_init_flow(&self, device_id: &str) -> Result<LoginFlow, KeygateError>;
    fn login_get_flow(
        &self,
        device_id: &str,
        login_flow_id: &str,
    ) -> Result<LoginFlow, KeygateError>;

    fn login_submit_password(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, KeygateError>;

    fn login_submit_magic_link(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, KeygateError>;

    fn login_submit_otp(
        &self,
        login_flow_id: &str,
        device_id: &str,
        magic_link: &str,
    ) -> Result<LoginFlow, KeygateError>;
}

impl Login for Keygate {
    fn login_init_flow(&self, device_id: &str) -> Result<LoginFlow, KeygateError> {
        todo!()
    }

    fn login_get_flow(
        &self,
        device_id: &str,
        login_flow_id: &str,
    ) -> Result<LoginFlow, KeygateError> {
        todo!()
    }

    fn login_submit_password(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, KeygateError> {
        todo!()
    }

    fn login_submit_magic_link(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, KeygateError> {
        todo!()
    }

    fn login_submit_otp(
        &self,
        login_flow_id: &str,
        device_id: &str,
        magic_link: &str,
    ) -> Result<LoginFlow, KeygateError> {
        todo!()
    }
}
