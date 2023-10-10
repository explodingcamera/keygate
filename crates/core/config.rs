use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Production,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StorageOptions {
    Sqlite { database_path: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// the domain keygate is running on, e.g `accounts.example.com`
    /// refresh tokens are only valid for this domain
    pub keygate_domain: String,

    /// admin api port
    /// if set to 0, the admin api will not be available
    pub admin_port: u16,

    /// admin api interface
    pub admin_interface: String,

    /// admin api prefix
    pub admin_prefix: Option<String>,

    /// public api port
    /// if set to 0, the api will not be available
    pub public_port: u16,

    /// public api interface
    pub public_interface: String,

    /// public api prefix
    pub public_prefix: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub environment: Environment,
    pub node_id: String,

    /// Options for the storage backend
    pub storage_options: StorageOptions,

    /// server configuration
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node_id: "__unset__".to_string(),
            environment: if cfg!(debug_assertions) {
                Environment::Development
            } else {
                Environment::Production
            },
            storage_options: StorageOptions::default(),
            server: ServerConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            admin_port: 8081,
            admin_interface: "127.0.0.1".to_string(),
            public_port: 8080,
            public_interface: "0.0.0.0".to_string(),
            keygate_domain: "auth.localhost".to_string(),
            admin_prefix: None,
            public_prefix: None,
        }
    }
}

impl Default for StorageOptions {
    fn default() -> Self {
        StorageOptions::Sqlite {
            database_path: "db.sql".into(),
        }
    }
}
