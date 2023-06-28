mod auth;
mod identity;

pub use auth::Auth;
pub use identity::Identity;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum APIError {
    #[error("The operation was cancelled.")]
    Cancelled(String),

    #[error("Unknown error.")]
    Unknown(String),

    #[error("Client specified an invalid argument.")]
    InvalidArgument(String),

    #[error("Some requested entity was not found.")]
    NotFound(String),

    #[error("Some entity that we attempted to create already exists.")]
    AlreadyExists(String),

    #[error("The caller does not have permission to execute the specified operation.")]
    PermissionDenied(String),

    #[error("The operation was aborted.")]
    Aborted(String),

    #[error("Operation is not implemented or not supported.")]
    Unimplemented(String),

    #[error("Internal error.")]
    Internal(String),

    #[error("The request does not have valid authentication credentials.")]
    Unauthenticated(String),
}

impl APIError {
    pub fn cancelled<T: Into<String>>(msg: T) -> Self {
        APIError::Cancelled(msg.into())
    }

    pub fn unknown<T: Into<String>>(msg: T) -> Self {
        APIError::Unknown(msg.into())
    }

    pub fn invalid_argument<T: Into<String>>(msg: T) -> Self {
        APIError::InvalidArgument(msg.into())
    }

    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        APIError::NotFound(msg.into())
    }

    pub fn already_exists<T: Into<String>>(msg: T) -> Self {
        APIError::AlreadyExists(msg.into())
    }

    pub fn permission_denied<T: Into<String>>(msg: T) -> Self {
        APIError::PermissionDenied(msg.into())
    }

    pub fn aborted<T: Into<String>>(msg: T) -> Self {
        APIError::Aborted(msg.into())
    }

    pub fn unimplemented<T: Into<String>>(msg: T) -> Self {
        APIError::Unimplemented(msg.into())
    }

    pub fn internal<T: Into<String>>(msg: T) -> Self {
        APIError::Internal(msg.into())
    }

    pub fn unauthenticated<T: Into<String>>(msg: T) -> Self {
        APIError::Unauthenticated(msg.into())
    }
}

impl From<prisma::client::QueryError> for APIError {
    fn from(err: prisma::client::QueryError) -> Self {
        match err {
            prisma::client::QueryError::Deserialize(message) => APIError::Internal(message),
            prisma::client::QueryError::Serialize(message) => APIError::Internal(message),
            error => APIError::Unknown(error.to_string()),
        }
    }
}

impl From<APIError> for tonic::Status {
    fn from(err: APIError) -> Self {
        match err {
            APIError::Cancelled(msg) => tonic::Status::cancelled(msg),
            APIError::Unknown(msg) => tonic::Status::unknown(msg),
            APIError::InvalidArgument(msg) => tonic::Status::invalid_argument(msg),
            APIError::NotFound(msg) => tonic::Status::not_found(msg),
            APIError::AlreadyExists(msg) => tonic::Status::already_exists(msg),
            APIError::PermissionDenied(msg) => tonic::Status::permission_denied(msg),
            APIError::Aborted(msg) => tonic::Status::aborted(msg),
            APIError::Unimplemented(msg) => tonic::Status::unimplemented(msg),
            APIError::Internal(msg) => tonic::Status::internal(msg),
            APIError::Unauthenticated(msg) => tonic::Status::unauthenticated(msg),
        }
    }
}

pub enum UserIdentifier {
    Email(String),
    Username(String),
    Id(String),
}

impl TryFrom<proto::api::identity::GetIdentityRequest> for UserIdentifier {
    type Error = APIError;

    fn try_from(value: proto::api::identity::GetIdentityRequest) -> Result<Self, Self::Error> {
        match value.user {
            Some(proto::api::identity::get_identity_request::User::Email(email)) => Ok(UserIdentifier::Email(email)),
            Some(proto::api::identity::get_identity_request::User::Username(username)) => Ok(UserIdentifier::Username(username)),
            Some(proto::api::identity::get_identity_request::User::Id(id)) => Ok(UserIdentifier::Id(id)),
            None => Err(APIError::invalid_argument("No user specified")),
        }
    }
}
