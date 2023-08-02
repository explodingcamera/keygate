#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

#[derive(Clone, Debug)]
pub struct IdentityConfig {
    pub signup_with_email: bool,
    pub signup_with_username: bool,
    pub signup_require_username: bool,
    pub signup_require_email: bool,
    pub signup_require_email_verification: bool,
    pub signup_process_lifetime: i64,

    pub login_with_username: bool,
    pub login_with_email: bool,
    pub login_process_lifetime: i64,

    pub allow_multiple_emails: bool,

    pub password_min_length: usize,
    pub check_leaked_passwords: bool,
}

#[derive(Clone, Debug)]
pub enum StorageOptions {
    Sqlite { database_path: String },
}

#[derive(Clone, Debug)]
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

pub type KeygateConfigInternal = std::sync::Arc<Configuration>;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub environment: Environment,

    /// Options for the storage backend
    pub storage_options: StorageOptions,

    /// server configuration
    pub server: ServerConfig,
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

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            allow_multiple_emails: false,
            signup_with_email: false,
            signup_with_username: true,
            signup_require_username: true,
            signup_require_email: false,
            signup_require_email_verification: false,
            signup_process_lifetime: 60 * 60,
            login_with_email: true,
            login_with_username: true,
            login_process_lifetime: 60 * 60,
            check_leaked_passwords: true,
            password_min_length: 8,
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
