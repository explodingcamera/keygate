#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

mod api;
pub mod models;
pub mod utils;

pub mod config;
use config::Configuration;
pub use config::Configuration as KeygateConfig;

mod storage;
use storage::StorageError;

pub use storage::constants as storage_constants;
pub use storage::traits;
pub use storage::Storage;
pub use storage::StorageType;
pub use storage::{RedisStorage, RocksDBStorage};

use thiserror::Error;

#[derive(Clone, Copy)]
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
    Identity(#[from] api::IdentityError),
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

    #[error("unknown error")]
    Unknown,
}

pub type KeygateResult<T> = Result<T, KeygateError>;
type KeygateConfigInternal = Arc<RwLock<Configuration>>;
type KeygateStorage = Arc<dyn Storage + Send + Sync>;

pub struct Keygate {
    pub config: KeygateConfigInternal,
    pub storage: KeygateStorage,
    pub health: Arc<RwLock<Health>>,

    pub identity: api::Identity,
    pub login: api::Login,
    pub metadata: api::Metadata,
    pub recovery: api::Recovery,
    pub session: api::Session,
    pub signup: api::Signup,
}

impl Keygate {
    pub async fn new(config: Configuration) -> Result<Keygate, KeygateError> {
        let res = match config.storage_type {
            StorageType::RocksDB => match RocksDBStorage::new() {
                Ok(storage) => Keygate::new_with_storage(config, Arc::new(storage)),
                Err(e) => return Err(e.into()),
            },
            StorageType::Redis => match RedisStorage::new().await {
                Ok(storage) => Keygate::new_with_storage(config, Arc::new(storage)),
                Err(e) => return Err(e.into()),
            },
        };

        Ok(res.await)
    }

    pub async fn new_with_storage(
        config: Configuration,
        storage: Arc<dyn Storage + Send + Sync>,
    ) -> Keygate {
        let config = Arc::new(RwLock::new(config));

        Keygate {
            config: config.clone(),
            storage: storage.clone(),
            health: Arc::new(RwLock::new(Health::Starting)),

            identity: api::Identity::new(config.clone(), storage.clone()).await,
            login: api::Login::new(config.clone(), storage.clone()).await,
            metadata: api::Metadata::new(config.clone(), storage.clone()).await,
            recovery: api::Recovery::new(config.clone(), storage.clone()).await,
            session: api::Session::new(config.clone(), storage.clone()).await,
            signup: api::Signup::new(config, storage.clone()).await,
        }
    }
}
