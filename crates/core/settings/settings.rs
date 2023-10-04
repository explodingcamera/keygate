use std::sync::{Arc, Mutex, OnceLock};

use dashmap::DashMap;
use keygate_utils::atomic::AtomicDateTime;
use thiserror::Error;
use time::{Duration, OffsetDateTime};

use crate::{
    database::{
        models::{Application, ApplicationSettings, GlobalSettings},
        DatabasePool,
    },
    KeygateInternal,
};

#[derive(Debug, Default)]
pub struct KeygateSettings {
    keygate: OnceLock<Arc<KeygateInternal>>,
    global: Mutex<Option<GlobalSettings>>,
    global_updated_at: AtomicDateTime, // should be a bit more efficient then in the mutex
    applications: DashMap<String, (ApplicationSettings, OffsetDateTime)>, // seperate dates here would be too much work lol
}

const GLOBAL_SETTINGS_UPDATE_INTERVAL: i64 = 60 * 2;
const APPLICATION_SETTINGS_UPDATE_INTERVAL: i64 = 60 * 2;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("poisoned lock: {0}")]
    LockPoisoned(String),

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("unknown error")]
    Unknown,
}

impl KeygateSettings {
    pub fn new() -> Self {
        Self::default()
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.get().expect("Keygate not initialized").db
    }

    pub async fn update_global(&self, settings: GlobalSettings) -> Result<(), SettingsError> {
        let new_settings = serde_json::to_string(&settings)?;
        let now = OffsetDateTime::now_utc();

        sqlx::query!(
            "UPDATE Application SET settings = $1, updated_at = $2 WHERE id = 'global'",
            new_settings,
            now
        )
        .execute(self.db())
        .await?;

        self.global_updated_at.set(OffsetDateTime::now_utc());
        self.global
            .lock()
            .map_err(|e| SettingsError::LockPoisoned(e.to_string()))?
            .replace(settings);

        Ok(())
    }

    pub async fn update_app(&self, application_id: &str, settings: ApplicationSettings) -> Result<(), SettingsError> {
        let new_settings = serde_json::to_string(&settings)?;
        let now = OffsetDateTime::now_utc();

        sqlx::query!(
            "UPDATE Application SET settings = $1, updated_at = $2 WHERE id = $3",
            new_settings,
            now,
            application_id
        )
        .execute(self.db())
        .await?;

        self.applications
            .insert(application_id.to_string(), (settings, OffsetDateTime::now_utc()));

        Ok(())
    }

    pub async fn global(&self) -> Result<GlobalSettings, SettingsError> {
        let global = {
            self.global
                .lock()
                .map_err(|e| SettingsError::LockPoisoned(e.to_string()))?
                .clone() // clone so we can drop the lock
        };

        let outdated = self.global_updated_at.get().unix_timestamp()
            < OffsetDateTime::now_utc().unix_timestamp() - GLOBAL_SETTINGS_UPDATE_INTERVAL;

        if global.is_none() || outdated {
            let app = sqlx::query_as!(Application, "SELECT * FROM Application WHERE id = 'global'")
                .fetch_optional(self.db())
                .await?;

            let new_global = match app {
                Some(app) => {
                    if app.id != "global" {
                        panic!("Global settings not found, this should not be possible");
                    }
                    serde_json::from_str(app.settings.as_str())?
                }
                None => {
                    let new_global = default_global_settings();
                    let settings = serde_json::to_string(&new_global)?;

                    sqlx::query!("INSERT INTO Application (id, settings) VALUES ('global', $1)", settings)
                        .execute(self.db())
                        .await?;

                    new_global
                }
            };

            self.global_updated_at.set(OffsetDateTime::now_utc());
            return Ok(new_global);
        }

        Ok(global
            .clone()
            .expect("Global settings is none, this should be impossible"))
    }

    pub async fn app(&self, application_id: &str) -> Result<Option<ApplicationSettings>, SettingsError> {
        let outdated = self
            .applications
            .get(application_id)
            .map(|d| {
                d.1.unix_timestamp() < OffsetDateTime::now_utc().unix_timestamp() - APPLICATION_SETTINGS_UPDATE_INTERVAL
            })
            .unwrap_or(true);

        if outdated {
            let app = match sqlx::query_as!(Application, "SELECT * FROM Application WHERE id = $1", application_id)
                .fetch_optional(self.db())
                .await?
            {
                Some(app) => app,
                None => return Ok(None),
            };

            let new_settings: ApplicationSettings = serde_json::from_str(app.settings.as_str())?;

            self.applications.insert(
                application_id.to_string(),
                (new_settings.clone(), OffsetDateTime::now_utc()),
            );

            return Ok(Some(new_settings));
        }

        Ok(Some(
            self.applications
                .get(application_id)
                .map(|app| app.0.clone())
                .expect("Application settings not found, this should not be possible"),
        ))
    }

    pub(crate) fn set_keygate(&self, keygate: Arc<KeygateInternal>) {
        self.keygate.set(keygate).unwrap();
    }
}

fn default_global_settings() -> GlobalSettings {
    GlobalSettings {
        login_identifier: crate::database::models::AccountNaming::UsernameOrEmail,
        check_haveibeenpwned: false,

        login_process_expires_in: Duration::minutes(5),
        signup_process_expires_in: Duration::minutes(5),

        default_refresh_token_expires_in: Duration::days(14),
        default_access_token_expires_in: Duration::minutes(5),

        email_verification: crate::database::models::EmailVerification::None,
        enable_multiple_emails_per_account: false,
        magic_link: None,

        minimum_age: None,
        require_birthdate: false,
        store_birthdate: false,

        require_full_name: false,
        signup_flow: crate::database::models::SignupFlow::UsernamePasswordAndEmail,
    }
}
