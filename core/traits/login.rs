use crate::{models::LoginFlow, KeySignal};

pub trait Login: Send + Sync {
    fn login_init_flow(&self, device_id: &str) -> Result<LoginFlow, String>;
    fn login_get_flow(&self, device_id: &str, login_flow_id: &str) -> Result<LoginFlow, String>;

    fn login_submit_password(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, String>;

    fn login_submit_magic_link(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, String>;

    fn login_submit_otp(
        &self,
        login_flow_id: &str,
        device_id: &str,
        magic_link: &str,
    ) -> Result<LoginFlow, String>;
}

impl Login for KeySignal {
    fn login_init_flow(&self, device_id: &str) -> Result<LoginFlow, String> {
        todo!()
    }

    fn login_get_flow(&self, device_id: &str, login_flow_id: &str) -> Result<LoginFlow, String> {
        todo!()
    }

    fn login_submit_password(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, String> {
        todo!()
    }

    fn login_submit_magic_link(
        &self,
        login_flow_id: &str,
        password: &str,
    ) -> Result<LoginFlow, String> {
        todo!()
    }

    fn login_submit_otp(
        &self,
        login_flow_id: &str,
        device_id: &str,
        magic_link: &str,
    ) -> Result<LoginFlow, String> {
        todo!()
    }
}
