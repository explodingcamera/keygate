use std::sync::Arc;

use proto::api::identity::{self, identity_service_server};
use proto::models;
use tonic::{Request, Response, Status};

use crate::KeygateInternal;
pub use proto::api::identity::identity_service_server::IdentityService;

#[derive(Debug, Clone)]
pub struct Identity {
    keygate: Arc<KeygateInternal>,
}

impl Identity {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    pub fn service(&self) -> identity_service_server::IdentityServiceServer<Identity> {
        identity_service_server::IdentityServiceServer::new(Self::new(self.keygate.clone()))
    }
}

#[tonic::async_trait]
impl IdentityService for Identity {
    async fn get(&self, request: Request<identity::GetIdentityRequest>) -> Result<Response<models::Identity>, Status> {
        Ok(Response::new(models::Identity {
            id: "test".to_string(),
            ..Default::default()
        }))
    }

    async fn create(&self, request: Request<models::Identity>) -> Result<Response<models::Identity>, Status> {
        // TODO: Implement create_identity function
        unimplemented!()
    }

    async fn update(&self, request: Request<models::Identity>) -> Result<Response<models::Identity>, Status> {
        // TODO: Implement update_identity function
        unimplemented!()
    }

    async fn delete(&self, request: Request<identity::DeleteIdentityRequest>) -> Result<Response<()>, Status> {
        // TODO: Implement delete_identity function
        unimplemented!()
    }

    async fn list(
        &self,
        request: Request<identity::ListIdentitiesRequest>,
    ) -> Result<Response<identity::ListIdentitiesResponse>, Status> {
        // TODO: Implement list_identities function
        unimplemented!()
    }
}
