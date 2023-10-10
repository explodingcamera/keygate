#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::sync::Arc;
use std::{fmt::Debug, path::Path};

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
use tracing::{info, warn};

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

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    APIError(#[from] api::APIError),

    #[error(transparent)]
    SettingsError(#[from] settings::SettingsError),
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
    pub async fn create_admin_app(&self) -> KeygateResult<()> {
        let app_created = self
            .inner
            .settings
            .create_application(
                "admin",
                database::models::ApplicationSettings {
                    access_token_expires_in: Some(time::Duration::minutes(10)),
                    refresh_token_expires_in: Some(time::Duration::days(1)),
                    access_token_format: database::models::TokenFormat::Jwt25519,
                },
            )
            .await?;

        match app_created {
            true => info!("Created admin application"),
            false => info!("Admin application already exists"),
        }

        Ok(())
    }

    pub async fn create_admin_user(&self) -> KeygateResult<()> {
        if self.identity.exists("admin").await? {
            info!("Admin user already exists");
            return Ok(());
        }

        let user_pw = keygate_utils::hash::password("admin")?;
        let user = self
            .identity
            .create(api::identity::CreateIdentity {
                username: Some("admin"),
                primary_email: Some("admin@keygate.io"),
                password_hash: Some(&user_pw),
            })
            .await?;

        Ok(())
    }

    pub async fn run(&self) -> KeygateResult<()> {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            self.inner.run().await?;
        }
    }

    #[allow(unreachable_patterns)]
    pub async fn new(config: Config) -> Result<Self, KeygateError> {
        sqlx::any::install_default_drivers();

        let db = match config.storage_options.clone() {
            config::StorageOptions::Sqlite { database_path } => {
                let mut path = database_path;

                if !path.starts_with("sqlite://") {
                    return Err(KeygateError::ValidationError("Invalid sqlite database path".into()));
                }

                if path.starts_with("sqlite://~") {
                    let home = dirs::home_dir().expect("Failed to get home directory");
                    path = path.replace('~', home.to_str().unwrap());
                }

                if !path.starts_with("sqlite://:memory:") {
                    let file_path = &path.strip_prefix("sqlite://").unwrap();
                    let file_path = Path::new(file_path);
                    std::fs::create_dir_all(file_path.parent().unwrap())?;
                    info!("Using sqlite database at {}", file_path.display());
                } else {
                    warn!("Using in-memory database. All data will be lost on restart.");
                }

                let db = DatabasePool::connect(&path).await?;
                sqlx::migrate!("./migrations")
                    .run(&db)
                    .await
                    .expect("Failed to run migrations");
                db
            }
            _ => panic!("Unsupported storage option"),
        };

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

        // ensure global settings exist
        internal
            .settings
            .global()
            .await
            .expect("Failed to load global settings");

        // ensure keypair exists
        internal.secrets.ensure_keypair().await;

        Keygate {
            inner: internal.clone(),
            identity: Arc::new(api::Identity::new(internal.clone())),
            auth: Arc::new(api::Auth::new(internal.clone())),
            session: Arc::new(api::Session::new(internal)),
        }
    }
}
