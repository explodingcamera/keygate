pub mod auth;
pub mod identity;
pub mod session;

pub use auth::Auth;
pub use identity::Identity;
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

impl APIError {
    fn not_found(text: &str) -> Self {
        Self::NotFound(text.to_string())
    }
    fn invalid_argument(text: &str) -> Self {
        Self::InvalidArgument(text.to_string())
    }
    fn internal(text: &str) -> Self {
        Self::Internal(text.to_string())
    }
}

pub enum UserIdentifier {
    Email(String),
    Username(String),
    Id(String),
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub enum SortBy {
    Email,
    Username,
    CreatedAt,
    LastActive,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub field: String,
    pub value: String,
}
