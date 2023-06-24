use std::sync::Arc;

use proto::{api::auth::auth_service_server::*, api::auth::*, models};
use sea_orm::prelude::*;
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
        unimplemented!()
    }

    async fn login_step(&self, request: Request<LoginStepRequest>) -> Result<Response<LoginResponse>, Status> {
        unimplemented!()
    }

    async fn login_status(
        &self,
        request: Request<LoginStatusRequest>,
    ) -> Result<Response<LoginStatusResponse>, Status> {
        unimplemented!()
    }
}
