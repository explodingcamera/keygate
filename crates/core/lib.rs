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
mod secrets;
use config::Configuration;
pub use config::Configuration as KeygateConfig;

mod storage;
pub use secrets::generate_ed25519_key_pair;
use secrets::SecretStore;
use storage::StorageError;

pub use storage::constants as storage_constants;
pub use storage::traits;
pub use storage::Storage;
pub use storage::StorageType;
pub use storage::{RedisStorage, SQLStorage};

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
    JWTError(#[from] keygate_jwt::JWTError),

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

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("poisoned lock: {0}")]
    LockPoisoned(String),

    #[error("unknown error")]
    Unknown,
}

pub type KeygateResult<T> = Result<T, KeygateError>;
pub type KeygateConfigInternal = Arc<RwLock<Configuration>>;
pub type KeygateSecretsStore = Arc<secrets::SecretStore>;
pub type KeygateSecrets = secrets::Secrets;
type KeygateStorage = Arc<dyn Storage + Send + Sync>;

pub struct Keygate {
    pub config: KeygateConfigInternal,
    pub storage: KeygateStorage,
    secrets: KeygateSecretsStore,

    pub health: Arc<RwLock<Health>>,

    pub identity: api::Identity,
    pub login: api::Login,
    pub metadata: api::Metadata,
    pub recovery: api::Recovery,
    pub session: api::Session,
    pub signup: api::Signup,
}

fn restrict_syscalls() {
    extrasafe::SafetyContext::new()
        .enable(extrasafe::builtins::basic::BasicCapabilities)
        .unwrap()
        .apply_to_all_threads()
        .unwrap();
}

impl Keygate {
    pub async fn new(config: Configuration, secrets: KeygateSecrets) -> Result<Keygate, KeygateError> {
        let storage_type = config.storage_type;
        let config = Arc::new(RwLock::new(config));
        let res = match storage_type {
            StorageType::SQL => match SQLStorage::new(config.clone()).await {
                Ok(storage) => Keygate::new_with_storage(config.clone(), Arc::new(storage), secrets),
                Err(e) => return Err(e.into()),
            },
            StorageType::Redis => match RedisStorage::new(config.clone()).await {
                Ok(storage) => Keygate::new_with_storage(config.clone(), Arc::new(storage), secrets),
                Err(e) => return Err(e.into()),
            },
        };

        Ok(res.await)
    }

    pub async fn new_with_storage(
        config: KeygateConfigInternal,
        storage: Arc<dyn Storage + Send + Sync>,
        secrets: KeygateSecrets,
    ) -> Keygate {
        let secrets_store = Arc::new(SecretStore::new(secrets));

        Keygate {
            config: config.clone(),
            storage: storage.clone(),
            secrets: secrets_store.clone(),
            health: Arc::new(RwLock::new(Health::Starting)),

            identity: api::Identity::new(config.clone(), storage.clone()).await,
            login: api::Login::new(config.clone(), storage.clone()).await,
            metadata: api::Metadata::new(config.clone(), storage.clone()).await,
            recovery: api::Recovery::new(config.clone(), storage.clone()).await,
            session: api::Session::new(config.clone(), storage.clone(), secrets_store).await,
            signup: api::Signup::new(config, storage.clone()).await,
        }
    }
}
