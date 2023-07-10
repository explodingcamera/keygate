use std::sync::Arc;

use keygate_core::Keygate;
use proto::api::identity::{self, identity_service_server::IdentityService};
use proto::models;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct IdentityServiceImpl {
    keygate: Arc<Keygate>,
}

impl IdentityServiceImpl {
    pub fn new(keygate: Arc<Keygate>) -> Self {
        Self { keygate }
    }
}

#[tonic::async_trait]
impl IdentityService for IdentityServiceImpl {
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
