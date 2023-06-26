use std::{net::IpAddr, sync::Arc};

use keygate_utils::random::secure_random_id;
use prisma::{identity, login_process, PrismaClient};
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

    // create a new login process for the given user
    async fn login(
        &self,
        // everything with an @ is considered an email
        username_or_email: &str,
        // ip_address has to be validated by the caller, can be empty (0.0.0.0) if not available
        ip_address: IpAddr,
    ) -> Result<LoginResponse, APIError> {
        let client = self.client();

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
                        true => identity::primary_email::equals,
                        false => identity::username::equals,
                    }(username_or_email.into()))
                    .exec()
                    .await?;

                let current_identity_id = current_identity.ok_or(APIError::not_found("User not found"))?.id;

                let login_process = login_process::Create {
                    id: _login_process_id,
                    expires_at: chrono::Utc::now().into(),
                    current_step: current_step.as_str_name().to_string(),
                    identity: identity::UniqueWhereParam::IdEquals(current_identity_id),
                    _params: vec![login_process::ip_address::set(Some(ip_address.to_string()))],
                };

                login_process.to_query(&client).exec().await?;

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

    async fn login_step(&self, process_id: &str, step_type: LoginStep, data: &str) -> Result<LoginResponse, APIError> {
        let next_steps: Vec<LoginStep> = self
            .client()
            .tx::<APIError, _, _, _>(|client| async move {
                let current_process = client
                    .login_process()
                    .find_unique(prisma::login_process::UniqueWhereParam::IdEquals(process_id.into()))
                    .exec()
                    .await?
                    .ok_or(APIError::not_found("Login process not found"));

                let current_step = LoginStep::from_str_name(&current_process?.current_step)
                    .ok_or(APIError::invalid_argument("Invalid step type"))?;

                match (current_step, step_type) {
                    (LoginStep::Email, LoginStep::Password) | (LoginStep::Username, LoginStep::Password) => {
                        // TODO: Implement password login
                        // TODO: Check if more steps are required (e.g. 2FA)
                        Ok(vec![LoginStep::Success])
                    }
                    _ => Err(APIError::invalid_argument("Invalid step type")),
                }
            })
            .await?;

        if next_steps.first() == Some(&LoginStep::Success) {
            Ok(LoginResponse {
                response: Some(login_response::Response::Success(LoginSuccessResponse {
                    refresh_token: "TODO".to_string(),
                })),
            })
        } else {
            Ok(LoginResponse {
                response: Some(login_response::Response::NextStep(LoginNextStepResponse {
                    step_type: next_steps.into_iter().map(|s| s as i32).collect(),
                    process_id: process_id.into(),
                })),
            })
        }
    }

    async fn login_status(&self, process_id: &str) -> Result<LoginStatusResponse, APIError> {
        let process = self
            .client()
            .login_process()
            .find_unique(prisma::login_process::UniqueWhereParam::IdEquals(process_id.into()))
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

    async fn account_exists(
        &self,
        // everything with an @ is considered an email
        username_or_email: &str,
    ) -> Result<AccountExistsResponse, APIError> {
        let exists = self
            .client()
            .identity()
            .find_unique(match username_or_email.contains('@') {
                true => identity::primary_email::equals,
                false => identity::username::equals,
            }(username_or_email.into()))
            .exec()
            .await?
            .is_some();

        Ok(AccountExistsResponse { exists })
    }

    async fn signup(&self, request: SignupRequest) -> Result<SignupResponse, APIError> {
        unimplemented!()
    }
}
