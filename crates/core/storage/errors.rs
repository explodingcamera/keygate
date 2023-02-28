use crate::utils::{encoding, validate::RefreshTokenError};
use std::sync::PoisonError;
use thiserror::Error;

use super::{redis::RedisStorageError, sql::SQLStorageError};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    SQLStorage(#[from] SQLStorageError),
    #[error(transparent)]
    RedisStorage(#[from] RedisStorageError),
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

impl<T> From<PoisonError<T>> for StorageError {
    fn from(err: PoisonError<T>) -> Self {
        Self::ConfigPoisoned
    }
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
