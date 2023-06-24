use std::time::Duration;

use crate::utils::{encoding, validate::RefreshTokenError};
use sea_orm::{error::DbErr as SQLStorageError, ConnectOptions, Database, DatabaseConnection};

use thiserror::Error;

#[derive(Debug)]
pub struct SQLStorageBackend {
    pub database: DatabaseConnection,
}

impl SQLStorageBackend {
    pub async fn connect() -> Result<Self, SQLStorageError> {
        let mut opt = ConnectOptions::new("protocol://username:password@host/database".to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            // .sqlx_logging(true)
            // .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

        let db = Database::connect(opt).await?;
        Ok(Self { database: db })
    }
}

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
