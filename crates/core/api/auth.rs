use std::{net::IpAddr, sync::Arc};

use keygate_utils::random::secure_random_id;
use prisma::{identity, PrismaClient};
use proto::api::auth::*;
use tonic::{Request, Response, Status};

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

    async fn login(&self, req: InitLoginRequest) -> Result<LoginResponse, APIError> {
        let client = self.client();

        let ip = req.ip_address;
        if ip.is_empty() || ip.parse::<IpAddr>().is_err() {
            return Err(APIError::invalid_argument("Invalid IP address"));
        }

        let login_process_id = secure_random_id();
        let _login_process_id = login_process_id.clone(); // we need to clone this for the closure

        let is_email = req.username_or_email.contains('@');
        let current_step = match is_email {
            true => LoginStep::Email,
            false => LoginStep::Username,
        };

        let next_steps = client
            ._transaction()
            .run::<APIError, _, _, _>(|client| async move {
                let current_identity = client
                    .identity()
                    .find_unique(match is_email {
                        true => identity::primary_email::equals(req.username_or_email),
                        false => identity::username::equals(req.username_or_email),
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

    async fn login_step(&self, request: Request<LoginStepRequest>) -> Result<Response<LoginResponse>, Status> {
        unimplemented!()
    }

    async fn login_status(&self, request: Request<LoginStatusRequest>) -> Result<Response<LoginStatusResponse>, Status> {
        unimplemented!()
    }

    async fn account_exists(&self, request: Request<AccountExistsRequest>) -> Result<Response<AccountExistsResponse>, Status> {
        unimplemented!()
    }

    async fn signup(&self, request: Request<SignupRequest>) -> Result<Response<SignupResponse>, Status> {
        unimplemented!()
    }
}
