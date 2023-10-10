use std::{net::IpAddr, sync::Arc};

use keygate_utils::{
    random::secure_random_id,
    tokens::{AccessToken, RefreshToken},
    validate::{is_valid_email, is_valid_password, is_valid_username},
};

use super::APIError;
use crate::{
    database::{
        models::{Identity, LoginProcess},
        DatabasePool,
    },
    KeygateInternal,
};

#[derive(Debug, Clone)]
pub struct Auth {
    keygate: Arc<KeygateInternal>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum LoginStep {
    Email,
    Username,
    Password,
}

impl LoginStep {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Username => "username",
            Self::Password => "password",
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        match name {
            "email" => Some(Self::Email),
            "username" => Some(Self::Username),
            "password" => Some(Self::Password),
            _ => None,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum LoginResponse {
    NextStep {
        step_type: Vec<LoginStep>,
        process_id: String,
    },
    Success {
        refresh_token: String,
    },
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginStatusResponse {
    pub current_step: String,
    pub expires_at: Option<time::OffsetDateTime>,
}

impl Auth {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.db
    }

    pub async fn verify_access_token(&self, token: &str) -> Result<AccessToken, APIError> {
        Ok(AccessToken {
            audience: "TODO".to_string(),
            issuer: "TODO".to_string(),
            key_id: "TODO".to_string(),
            session_id: "TODO".to_string(),
            subject: "TODO".to_string(),
        })
    }

    pub async fn verify_refresh_token(&self, token: &str) -> Result<RefreshToken, APIError> {
        Ok(RefreshToken {
            audience: "TODO".to_string(),
            issuer: "TODO".to_string(),
            key_id: "TODO".to_string(),
            session_id: "TODO".to_string(),
            subject: "TODO".to_string(),
        })
    }

    // create a new login process for the given user
    pub async fn login_create(
        &self,
        // everything with an @ is considered an email
        username_or_email: &str,
        // ip_address has to be validated by the caller, can be empty (0.0.0.0) if not available
        ip_address: Option<IpAddr>,
    ) -> Result<LoginResponse, APIError> {
        let login_process_id = secure_random_id();
        let now = time::OffsetDateTime::now_utc();
        let is_email = username_or_email.contains('@');
        let ip_address = ip_address.map(|ip| ip.to_string());
        let current_step = match is_email {
            true => LoginStep::Email,
            false => LoginStep::Username,
        }
        .as_str_name();

        let next_steps = {
            let mut tx = self.db().begin().await?;
            let field = match is_email {
                true => "primary_email",
                false => "username",
            };

            let current_identity_id = sqlx::query!("SELECT id FROM Identity WHERE $1 = $2", field, username_or_email)
                .fetch_optional(&mut *tx)
                .await?
                .ok_or(APIError::not_found("User not found"))?
                .id;

            sqlx::query!(
                "INSERT INTO LoginProcess (id, expires_at, current_step, identity_id, ip_address) VALUES ($1, $2, $3, $4, $5)",
                login_process_id,
                now,
                current_step,
                current_identity_id,
                ip_address
            )
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;

            // next step is always password for now
            // TODO: device login
            vec![LoginStep::Password]
        };

        Ok(LoginResponse::NextStep {
            step_type: next_steps,
            process_id: login_process_id,
        })
    }

    pub async fn login_step(
        &self,
        process_id: &str,
        step_type: LoginStep,
        data: &str,
    ) -> Result<LoginResponse, APIError> {
        let next_steps: Option<Vec<LoginStep>> = {
            let mut tx = self.db().begin().await?;

            let current_process = sqlx::query!(
                "SELECT current_step, identity_id FROM LoginProcess WHERE id = $1",
                process_id
            )
            .fetch_one(&mut *tx)
            .await?;

            let current_step = LoginStep::from_str_name(&current_process.current_step)
                .ok_or(APIError::invalid_argument("Invalid step type"))?;

            match (current_step, step_type) {
                (LoginStep::Email, LoginStep::Password) | (LoginStep::Username, LoginStep::Password) => {
                    let identity = sqlx::query_as!(
                        Identity,
                        "SELECT * FROM Identity WHERE id = $1",
                        current_process.identity_id
                    )
                    .fetch_one(&mut *tx)
                    .await?;

                    let password_hash = identity
                        .password_hash
                        .ok_or(APIError::invalid_argument("Password not set"))?;
                    if !keygate_utils::hash::verify(data, &password_hash)
                        .map_err(|e| APIError::internal(&format!("Failed to verify password: {}", e)))?
                    {
                        return Err(APIError::invalid_argument("Invalid password"));
                    }

                    // TODO: Check if more steps are required (e.g. 2FA)
                    None
                }
                _ => return Err(APIError::invalid_argument("Invalid step type")),
            }
        };

        match next_steps {
            None => Ok(LoginResponse::Success {
                refresh_token: "TODO".to_string(),
            }),
            Some(next_steps) => Ok(LoginResponse::NextStep {
                step_type: next_steps,
                process_id: process_id.into(),
            }),
        }
    }

    pub async fn login_status(&self, process_id: &str) -> Result<LoginStatusResponse, APIError> {
        let process = sqlx::query_as!(LoginProcess, "SELECT * FROM LoginProcess WHERE id = $1", process_id)
            .fetch_one(self.db())
            .await?;

        let _ =
            LoginStep::from_str_name(&process.current_step).ok_or(APIError::invalid_argument("Invalid step type"))?;

        Ok(LoginStatusResponse {
            current_step: process.current_step.to_string(),
            expires_at: process.expires_at,
        })
    }

    pub async fn signup(
        &self,
        username: &str,
        password: &str,
        email: &str,
        ip_address: IpAddr,
    ) -> Result<Identity, APIError> {
        if !is_valid_username(username) {
            return Err(APIError::invalid_argument("Invalid username"));
        }

        if !is_valid_password(password) {
            return Err(APIError::invalid_argument("Invalid password"));
        }

        if !is_valid_email(email) {
            return Err(APIError::invalid_argument("Invalid email"));
        }

        let user_id = secure_random_id();
        let password_hash = keygate_utils::hash::password(password)
            .map_err(|e| APIError::internal(&format!("Failed to hash password: {}", e)))?;

        let new_user = sqlx::query_as!(
            Identity,
            r#"
                INSERT INTO Identity (id, username, primary_email, password_hash)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            user_id,
            username,
            email,
            password_hash
        )
        .fetch_one(self.db())
        .await?;

        Ok(new_user)
    }
}
