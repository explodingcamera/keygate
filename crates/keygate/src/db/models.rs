use time::OffsetDateTime;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoginMethod {
    EmailAndPassword,
    UsernameAndPassword,
    EmailMagicLink,
    IdentityProvider(String),
    Passkey,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SignupMethod {
    Default,
    IdentityProvider(String),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Requirement {
    Required,
    Optional,
    Disabled,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Settings {
    pub login_enabled_methods: Vec<LoginMethod>,
    pub signup_enabled: bool,
    pub signup_enabled_methods: Vec<SignupMethod>,
    pub signup_require_email_verification: bool, // email is always required
    pub signup_username: Requirement,
    pub signup_password: Requirement,
    pub username_len: (usize, usize),
    pub username_regex: String,
    pub password_min_len: usize,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Identity {
    pub username: Option<String>,
    pub primary_email: Option<String>,
    pub password_hash: Option<String>,
    pub last_login: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Email {
    pub email: String,
    pub identity_id: String,
    pub verified: bool,

    pub verification_code: Option<String>,
    pub verification_code_expires_at: Option<OffsetDateTime>,
    pub verification_attempts: Vec<OffsetDateTime>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MagicLink {
    pub email: String,
    pub identity_id: String,
    pub token: String,
    pub expires_at: OffsetDateTime,
    pub used_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[derive(Debug, PartialEq, Eq)]
pub struct Session {
    pub refresh_token: String,
    pub identity_id: String,
    pub ips: Vec<String>,

    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
}
