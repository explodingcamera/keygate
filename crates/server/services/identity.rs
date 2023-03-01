use proto::v1::api::identity::{self, identity_service_server::IdentityService, GetIdentityResponse};
use proto::v1::models;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct IdentityServiceImpl {}

#[tonic::async_trait]
impl IdentityService for IdentityServiceImpl {
    async fn get_identity(
        &self,
        request: Request<identity::GetIdentityRequest>,
    ) -> Result<Response<identity::GetIdentityResponse>, Status> {
        let response = identity::GetIdentityResponse {
            identity: Some(models::Identity {
                id: "test".to_string(),
                ..Default::default()
            }),
        };
        Ok(Response::new(response))
    }

    async fn create_identity(
        &self,
        request: Request<identity::CreateIdentityRequest>,
    ) -> Result<Response<identity::CreateIdentityResponse>, Status> {
        // TODO: Implement create_identity function
        unimplemented!()
    }

    async fn update_identity(
        &self,
        request: Request<identity::UpdateIdentityRequest>,
    ) -> Result<Response<identity::UpdateIdentityResponse>, Status> {
        // TODO: Implement update_identity function
        unimplemented!()
    }

    async fn delete_identity(
        &self,
        request: Request<identity::DeleteIdentityRequest>,
    ) -> Result<Response<identity::DeleteIdentityResponse>, Status> {
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
