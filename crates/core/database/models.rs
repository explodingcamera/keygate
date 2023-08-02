use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, FromRow};
use time::Duration;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_active: OffsetDateTime,
    pub username: Option<String>,
    pub primary_email: Option<String>,

    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
    pub identity_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub verified: bool,
    pub last_verification_request: Option<OffsetDateTime>,
    pub verification_code: Option<String>,
    pub verification_code_expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LinkedAccount {
    pub id: String,
    pub provider_id: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LoginProcess {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub identity_id: String,
    pub ip_address: Option<String>,
    pub expires_at: Option<OffsetDateTime>,
    pub completed: bool,
    pub current_step: String,
    pub magic_link: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PublicKey {
    pub id: i32,
    pub created_at: OffsetDateTime,
    pub key_type: String,
    pub node_id: String,
    pub valid_until: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Meta {
    pub key: String,
    pub updated_at: OffsetDateTime,
    pub value: String,
    pub value_date: Option<OffsetDateTime>,
    pub value_int: Option<i32>,
    pub value_bool: Option<bool>,
    pub value_byte: Option<Vec<u8>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub identity_id: String,
    pub session_id: Option<String>,
    pub node_id: String,
    pub action: String,
    pub target_id: Option<String>,
    pub target_type: Option<String>,
    pub data: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub initial_ip_address: String,
    pub node_id: String,
    pub refresh_token: String,
    pub token_type: String,
    pub identity_id: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct APIKey {
    pub key: String,
    pub identity_id: Option<String>,
    pub name: String,
    pub target: String,
    pub audience: String,
    pub public: bool,
    pub hostnames: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountNaming {
    Username,
    Email,
    UsernameOrEmail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicLinkSettings {
    pub token_expires_in: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailVerification {
    None,
    Optional { token_expires_in: Duration },
    RequiredForLogin { token_expires_in: Duration },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignupFlow {
    UsernamePasswordAndEmail,
    UsernamePassword,
    EmailPassword,
    EmailOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub default_access_token_expires_in: Duration,
    pub default_refresh_token_expires_in: Duration,

    pub login_process_expires_in: Duration,
    pub signup_process_expires_in: Duration,

    pub signup_flow: SignupFlow,

    // if this is false, and minimum_age is set, the user will be asked if they are over the minimum age
    pub require_birthdate: bool,
    pub store_birthdate: bool,
    pub minimum_age: Option<i32>,

    pub require_full_name: bool,

    pub email_verification: EmailVerification,
    pub enable_multiple_emails_per_account: bool,
    pub check_haveibeenpwned: bool,

    /// What identifier to use for login
    pub login_identifier: AccountNaming,

    pub magic_link: Option<MagicLinkSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenFormat {
    JWT,
    PASETOV4,
    Biscuit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub access_token_format: TokenFormat,
    pub access_token_expires_in: Option<Duration>,
    pub refresh_token_expires_in: Option<Duration>,
}
