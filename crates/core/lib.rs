#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

use config::KeygateConfigInternal;
pub use proto::models;

mod api;
pub mod utils;

pub mod config;
mod secrets;
use arc_swap::ArcSwap;
use config::Configuration;
pub use config::Configuration as KeygateConfig;

mod storage;
use sea_orm::DatabaseConnection;
pub use secrets::generate_ed25519_key_pair;
use storage::StorageError;

use thiserror::Error;

#[derive(Clone, Copy, Debug)]
pub enum Health {
    Healthy,
    Starting,
    Unhealthy,
}

#[derive(Error, Debug)]
pub enum KeygateError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    JWTError(#[from] keygate_jwt::JWTError),

    #[error(transparent)]
    Login(#[from] api::LoginError),
    #[error(transparent)]
    Metadata(#[from] api::MetadataError),
    #[error(transparent)]
    Recovery(#[from] api::RecoveryError),
    #[error(transparent)]
    Session(#[from] api::SessionError),
    #[error(transparent)]
    Signup(#[from] api::SignupError),

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("poisoned lock: {0}")]
    LockPoisoned(String),

    #[error("unknown error")]
    Unknown,
}

pub type KeygateResult<T> = Result<T, KeygateError>;

pub type KeygateSecretsStore = Arc<secrets::SecretStore>;
pub type KeygateSecrets = secrets::Secrets;
pub type KeygateSql = Arc<storage::SQLStorageBackend>;

#[derive(Debug)]
struct KeygateInternal {
    pub config: KeygateConfigInternal,
    pub sql: KeygateSql,
    pub health: ArcSwap<Health>,
}

impl KeygateInternal {
    fn db(&self) -> &DatabaseConnection {
        &self.sql.database
    }
}

#[derive(Debug)]
pub struct Keygate {
    inner: Arc<KeygateInternal>,
    pub identity: Arc<api::Identity>,
}

impl Keygate {
    pub async fn new(config: Configuration, secrets: KeygateSecrets) -> Result<Self, KeygateError> {
        let config = Arc::new(RwLock::new(config));
        unimplemented!();
        // Keygate::new_with_storage(config, storage, secrets).await
        // Ok(res.await)
    }

    pub async fn new_with_storage(config: KeygateConfigInternal, sql: KeygateSql, secrets: KeygateSecrets) -> Self {
        let internal = Arc::new(KeygateInternal {
            config,
            sql,
            health: ArcSwap::from_pointee(Health::Starting),
        });

        Keygate {
            inner: internal.clone(),
            identity: Arc::new(api::Identity::new(internal)),
        }
    }
}
