#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::sync::Arc;

pub mod api;
pub mod database;
pub mod settings;

pub mod config;
mod secrets;
use arc_swap::ArcSwap;
use config::Config;
pub use config::Config as KeygateConfig;

use database::DatabasePool;
use secrets::Secrets;
use settings::KeygateSettings;
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
pub enum Health {
    Healthy,
    Starting,
    Unhealthy,
}

#[derive(Error, Debug)]
pub enum KeygateError {
    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("poisoned lock: {0}")]
    LockPoisoned(String),

    #[error("unknown error")]
    Unknown,

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

pub type KeygateResult<T> = Result<T, KeygateError>;

#[derive(Debug)]
pub(crate) struct KeygateInternal {
    pub config: Arc<Config>,
    pub secrets: Arc<Secrets>,
    pub db: DatabasePool,
    pub health: ArcSwap<Health>,
    pub settings: KeygateSettings,
}

impl KeygateInternal {
    pub async fn run(&self) -> KeygateResult<()> {
        self.secrets.run().await;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Keygate {
    inner: Arc<KeygateInternal>,
    pub auth: Arc<api::Auth>,
    pub session: Arc<api::Session>,
    pub identity: Arc<api::Identity>,
}

impl Keygate {
    pub async fn run(&self) -> KeygateResult<()> {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            self.inner.run().await?;
        }
    }

    pub async fn new(config: Config) -> Result<Self, KeygateError> {
        sqlx::any::install_default_drivers();
        let db = DatabasePool::connect("sqlite://:memory:").await?;
        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .expect("Failed to run migrations");

        Ok(Keygate::new_with_storage(config, db).await)
    }

    pub async fn new_with_storage(config: Config, db: DatabasePool) -> Self {
        let internal = Arc::new(KeygateInternal {
            config: Arc::new(config),
            secrets: Arc::new(Secrets::new()),
            db,
            health: ArcSwap::from_pointee(Health::Starting),
            settings: KeygateSettings::new(),
        });

        internal.settings.set_keygate(internal.clone());
        internal.secrets.set_keygate(internal.clone());

        Keygate {
            inner: internal.clone(),
            identity: Arc::new(api::Identity::new(internal.clone())),
            auth: Arc::new(api::Auth::new(internal.clone())),
            session: Arc::new(api::Session::new(internal)),
        }
    }
}
