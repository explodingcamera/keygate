use crate::{
    models,
    storage::StorageError,
    utils::tokens::{self},
    KeygateConfigInternal, KeygateError, KeygateSecretsStore, KeygateSql,
};
use chrono::{DateTime, Utc};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("unknown error")]
    Unknown,
    #[error("invalid claims")]
    InvalidClaims,
}

#[derive(Debug)]
pub struct Session {
    config: KeygateConfigInternal,
    storage: KeygateSql,
    secrets: KeygateSecretsStore,
}

impl Session {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateSql, secrets: KeygateSecretsStore) -> Self {
        Self {
            config,
            storage,
            secrets,
        }
    }
}

type RefreshTokenExpiration = DateTime<Utc>;
type AccessTokenExpiration = DateTime<Utc>;
fn get_expiration_times(
    config: KeygateConfigInternal,
) -> Result<(RefreshTokenExpiration, AccessTokenExpiration), StorageError> {
    let token_config = config.token.clone();

    let now = chrono::Utc::now();
    let refresh_expires_at = now + chrono::Duration::seconds(token_config.refresh_token_lifetime);
    let access_expires_at = now + chrono::Duration::seconds(token_config.access_token_lifetime);

    Ok((refresh_expires_at, access_expires_at))
}

impl Session {
    pub async fn create(&self, identity_id: &str) -> Result<(tokens::RefreshToken, tokens::AccessToken), KeygateError> {
        todo!();

        // let (refresh_expires_at, access_expires_at) = get_expiration_times(self.config.clone())?;
        // let (refresh_token, session) = self.storage.session_create(identity_id, refresh_expires_at).await?;

        // let access_token = tokens::AccessToken::generate(
        //     &session.identity_id,
        //     "keygate-js",
        //     access_expires_at.timestamp(),
        //     self.secrets.jwt_ed25519_keypair()?,
        // )?;
        // let refresh_token = tokens::RefreshToken::new(&refresh_token.id);

        // Ok((refresh_token, access_token))
    }

    pub async fn validate(&self, access_token: &str) -> Result<models::Session, KeygateError> {
        todo!();

        // let claims = self
        //     .secrets
        //     .jwt_ed25519_keypair()?
        //     .public_key()
        //     .verify_token::<NoCustomClaims>(access_token, None)
        //     .map_err(KeygateError::from)?;

        // let claims: KeygateClaims = claims
        //     .try_into()
        //     .map_err(|_| KeygateError::from(SessionError::InvalidClaims))?;

        // if claims.expires_at.as_secs() < chrono::Utc::now().timestamp().unsigned_abs() {
        //     return Err(JWTError::TokenHasExpired.into());
        // }

        // let Some(session) = self.storage.session_by_id(&claims.jwt_id).await? else {
        //     return Err(SessionError::NotFound.into());
        // };

        // if session.revoked_at.is_some() {
        //     return Err(JWTError::OldTokenReused.into());
        // }

        // Ok(session)
    }

    pub async fn invalidate(&self, access_token_id: &str) -> Result<(), KeygateError> {
        todo!();
        // Ok(self.storage.access_token_revoke(access_token_id).await?)
    }

    pub async fn all(&self, identity_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn refresh_invalidate(&self, refresh_token_id: &str) -> Result<(), KeygateError> {
        todo!();
        // Ok(self.storage.refresh_token_revoke(refresh_token_id).await?)
    }

    pub async fn refresh(
        &self,
        refresh_token_id: &str,
    ) -> Result<(tokens::AccessToken, tokens::RefreshToken), KeygateError> {
        todo!();
        // let (refresh_expires_at, access_expires_at) = get_expiration_times(self.config.clone())?;

        // let (refresh_token, session) = self
        //     .storage
        //     .refresh_token_rotate(refresh_token_id, refresh_expires_at, access_expires_at)
        //     .await?;

        // let access_token = tokens::AccessToken::generate(
        //     &session.identity_id,
        //     "keygate-js",
        //     access_expires_at.timestamp(),
        //     self.secrets.jwt_ed25519_keypair()?,
        // )?;

        // let refresh_token = tokens::RefreshToken::new(&refresh_token.id);

        // Ok((access_token, refresh_token))
    }
}
