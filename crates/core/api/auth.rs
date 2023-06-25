use std::{net::IpAddr, sync::Arc};

use proto::{api::auth::auth_service_server::*, api::auth::*};
use tonic::{Request, Response, Status};

use crate::KeygateInternal;

#[derive(Debug, Clone)]
pub struct Auth {
    keygate: Arc<KeygateInternal>,
}

impl Auth {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    pub fn service(&self) -> AuthServiceServer<Auth> {
        AuthServiceServer::new(Self::new(self.keygate.clone()))
    }
}

#[tonic::async_trait]
impl AuthService for Auth {
    async fn login(&self, request: Request<InitLoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let db = self.keygate.db();

        let ip = req.ip_address;
        if ip.is_empty() || ip.parse::<IpAddr>().is_err() {
            return Err(Status::invalid_argument("Invalid IP address"));
        }

        let login_process_id = keygate_utils::secure_random_id();
        let is_email = req.username_or_email.contains('@');
        let current_step = match is_email {
            true => LoginStep::Email,
            false => LoginStep::Username,
        };

        let next_steps = db
            .transaction::<_, Vec<LoginStep>, DbErr>(|txn| {
                let login_process_id = login_process_id.clone();
                Box::pin(async move {
                    let user = database::Identity::find()
                        .filter(match is_email {
                            true => database::identity::Column::PrimaryEmail.eq(req.username_or_email),
                            false => database::identity::Column::Username.eq(req.username_or_email),
                        })
                        .one(txn)
                        .await?
                        .ok_or(DbErr::Custom("User not found".into()))?;

                    let login_process: database::login::ActiveModel = database::login::Model {
                        current_step: current_step.as_str_name().to_string(),
                        id: login_process_id.clone(),
                        identity_id: user.id.clone(),
                        completed: false,
                        created_at: chrono::Utc::now().timestamp(),
                        expires_at: chrono::Utc::now().timestamp(),
                        updated_at: chrono::Utc::now().timestamp(),
                        magic_link: None,
                        ip_address: Some(ip),
                    }
                    .into();

                    login_process.save(txn).await?;

                    // for now, we only support password login
                    Ok(vec![LoginStep::Password])
                })
            })
            .await
            .map_err(|_| Status::internal("Database error"))?;

        Ok(Response::new(LoginResponse {
            response: Some(login_response::Response::NextStep(LoginNextStepResponse {
                step_type: next_steps.iter().map(|s| *s as i32).collect(),
                process_id: login_process_id,
            })),
        }))
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
