mod auth;
mod identity;
mod session;

pub use auth::Auth;
pub use identity::IdentityAPI;
pub use session::Session;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum APIError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

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

pub enum UserIdentifier {
    Email(String),
    Username(String),
    Id(String),
}
