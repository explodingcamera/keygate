use std::sync::Arc;

use proto::api::identity::{self, get_identity_request, identity_service_server};
use proto::models;
use sea_orm::prelude::*;
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
        let db = self.keygate.db();
        let req = request.into_inner();
        let user = req.user.ok_or(Status::invalid_argument("Must provide user"))?;

        let identity = match user {
            get_identity_request::User::Id(id) => database::Identity::find_by_id(id),
            get_identity_request::User::Email(email) => {
                database::Identity::find().filter(database::identity::Column::PrimaryEmail.eq(email))
            }
            get_identity_request::User::Username(username) => {
                database::Identity::find().filter(database::identity::Column::Username.eq(username))
            }
        }
        .one(db)
        .await
        .map_err(|_| Status::not_found("Identity not found"))?
        .ok_or(Status::not_found("Identity not found"))?;

        match req.include_private_fields {
            Some(true) => Ok(Response::new(identity.into())),
            _ => Ok(Response::new(identity.into())),
        }
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

    async fn list(&self, request: Request<identity::ListIdentitiesRequest>) -> Result<Response<identity::ListIdentitiesResponse>, Status> {
        // TODO: Implement list_identities function
        unimplemented!()
    }
}
