use crate::utils::{encoding, validate::RefreshTokenError};
use sea_orm::error::DbErr as SQLStorageError;

use thiserror::Error;

pub trait StorageBacked {}

pub struct SQLStorage {}

impl SQLStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl StorageBacked for SQLStorage {}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    SQLStorage(#[from] SQLStorageError),
    #[error(transparent)]
    RefreshToken(#[from] RefreshTokenError),
    #[error("invalid session: {0}")]
    Session(String),
    #[error(transparent)]
    Decoding(#[from] encoding::EncodingError),
    #[error(transparent)]
    Storage(#[from] LogicStorageError),
    #[error("paniced at {0}")]
    Panic(String),
    #[error("config poisoned")]
    ConfigPoisoned,
}

#[derive(Error, Debug)]
pub enum LogicStorageError {
    #[error("the key {0} already exists")]
    AlreadyExists(String),
    #[error("the key {0} wasn't found")]
    NotFound(String),
    #[error("unknown storage error")]
    Unknown,
}
