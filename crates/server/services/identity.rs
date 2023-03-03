use proto::v1::api::identity::{self, identity_service_server::IdentityService};
use proto::v1::models;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct IdentityServiceImpl {}

#[tonic::async_trait]
impl IdentityService for IdentityServiceImpl {
    async fn get_identity(
        &self,
        request: Request<identity::GetIdentityRequest>,
    ) -> Result<Response<models::Identity>, Status> {
        Ok(Response::new(models::Identity {
            id: "test".to_string(),
            ..Default::default()
        }))
    }

    async fn create_identity(&self, request: Request<models::Identity>) -> Result<Response<models::Identity>, Status> {
        // TODO: Implement create_identity function
        unimplemented!()
    }

    async fn update_identity(&self, request: Request<models::Identity>) -> Result<Response<models::Identity>, Status> {
        // TODO: Implement update_identity function
        unimplemented!()
    }

    async fn delete_identity(&self, request: Request<identity::DeleteIdentityRequest>) -> Result<Response<()>, Status> {
        // TODO: Implement delete_identity function
        unimplemented!()
    }

    async fn list_identities(
        &self,
        request: Request<identity::ListIdentitiesRequest>,
    ) -> Result<Response<identity::ListIdentitiesResponse>, Status> {
        // TODO: Implement list_identities function
        unimplemented!()
    }
}
