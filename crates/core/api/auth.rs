use std::{net::IpAddr, sync::Arc};

use keygate_utils::random::secure_random_id;
use prisma::{identity, PrismaClient};
use proto::api::auth::*;

use super::APIError;
use crate::KeygateInternal;

#[derive(Debug, Clone)]
pub struct Auth {
    keygate: Arc<KeygateInternal>,
}

impl Auth {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn client(&self) -> &PrismaClient {
        &self.keygate.prisma
    }

    async fn login(&self, username_or_email: String, ip_address: String) -> Result<LoginResponse, APIError> {
        let client = self.client();

        if ip_address.is_empty() || ip_address.parse::<IpAddr>().is_err() {
            return Err(APIError::invalid_argument("Invalid IP address"));
        }

        let login_process_id = secure_random_id();
        let _login_process_id = login_process_id.clone(); // we need to clone this for the closure

        let is_email = username_or_email.contains('@');
        let current_step = match is_email {
            true => LoginStep::Email,
            false => LoginStep::Username,
        };

        let next_steps = client
            .tx::<APIError, _, _, _>(|client| async move {
                let current_identity = client
                    .identity()
                    .find_unique(match is_email {
                        true => identity::primary_email::equals(username_or_email),
                        false => identity::username::equals(username_or_email),
                    })
                    .exec()
                    .await?
                    .ok_or(APIError::not_found("User not found"))?;

                client
                    .login_process()
                    .create(
                        _login_process_id,
                        chrono::Utc::now().into(),
                        current_step.as_str_name().to_string(),
                        prisma::identity::UniqueWhereParam::IdEquals(current_identity.id),
                        vec![],
                    )
                    .exec()
                    .await?;

                // next step is always password for now
                // TODO: device login
                Ok(vec![LoginStep::Password as i32])
            })
            .await?;

        Ok(LoginResponse {
            response: Some(login_response::Response::NextStep(LoginNextStepResponse {
                step_type: vec![LoginStep::Password as i32],
                process_id: login_process_id,
            })),
        })
    }

    async fn login_step(&self, request: LoginStepRequest) -> Result<LoginResponse, APIError> {
        let next_steps: Option<Vec<LoginStep>> = self
            .client()
            .tx::<APIError, _, _, _>(|client| async move {
                let current_process = client
                    .login_process()
                    .find_unique(prisma::login_process::UniqueWhereParam::IdEquals(request.process_id))
                    .exec()
                    .await?
                    .ok_or(APIError::not_found("Login process not found"))?;

                let current_step = LoginStep::from_str_name(&current_process.current_step)
                    .ok_or(APIError::invalid_argument("Invalid step type"))?;
                let step = LoginStep::from_i32(request.step_type).ok_or(APIError::invalid_argument("Invalid step type"))?;

                match (current_step, step) {
                    (LoginStep::Email, LoginStep::Password) | (LoginStep::Username, LoginStep::Password) => {
                        // TODO: Implement password login
                    }
                    _ => return Err(APIError::invalid_argument("Invalid step type")),
                };

                // TODO: Check if more steps are required (e.g. 2FA)
                Ok(None)
            })
            .await?;

        unimplemented!()
    }

    async fn login_status(&self, request: LoginStatusRequest) -> Result<LoginStatusResponse, APIError> {
        let process = self
            .client()
            .login_process()
            .find_unique(prisma::login_process::UniqueWhereParam::IdEquals(request.process_id))
            .exec()
            .await?
            .ok_or(APIError::not_found("Login process not found"))?;

        Ok(LoginStatusResponse {
            current_step: LoginStep::from_str_name(&process.current_step)
                .ok_or(APIError::invalid_argument("Invalid step type"))?
                .into(),
            expires_at: process.expires_at.timestamp(),
        })
    }

    async fn account_exists(&self, request: AccountExistsRequest) -> Result<AccountExistsResponse, APIError> {
        let exists = self
            .client()
            .identity()
            .find_unique(match request.request.ok_or(APIError::invalid_argument("Missing request"))? {
                account_exists_request::Request::Email(email) => identity::primary_email::equals(email),
                account_exists_request::Request::Username(username) => identity::username::equals(username),
            })
            .exec()
            .await?
            .is_some();

        Ok(AccountExistsResponse { exists })
    }

    async fn signup(&self, request: SignupRequest) -> Result<SignupResponse, APIError> {
        unimplemented!()
    }
}
