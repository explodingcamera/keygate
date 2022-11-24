use crate::{
    models,
    storage::StorageError,
    utils::tokens::{self, KeygateClaims, UnsignedAccessToken},
    KeygateConfigInternal, KeygateError, KeygateStorage,
};
use chrono::{DateTime, Utc};
use keygate_jwt::JWTError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("unknown error")]
    Unknown,
}

pub struct Session {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Session {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

type RefreshTokenExpiration = DateTime<Utc>;
type AccessTokenExpiration = DateTime<Utc>;
fn get_expiration_times(
    config: KeygateConfigInternal,
) -> Result<(RefreshTokenExpiration, AccessTokenExpiration), StorageError> {
    let token_config = config.read().map_err(StorageError::from)?.token.clone();

    let now = chrono::Utc::now();
    let refresh_expires_at = now + chrono::Duration::seconds(token_config.refresh_token_lifetime);
    let access_expires_at = now + chrono::Duration::seconds(token_config.access_token_lifetime);

    Ok((refresh_expires_at, access_expires_at))
}

impl Session {
    pub async fn create(
        &self,
        identity_id: &str,
    ) -> Result<(tokens::RefreshToken, tokens::AccessToken), KeygateError> {
        let (refresh_expires_at, access_expires_at) = get_expiration_times(self.config.clone())?;
        let (access_token, refresh_token, _) = self
            .storage
            .create_session(identity_id, refresh_expires_at, access_expires_at)
            .await?;

        let access_token: tokens::AccessToken =
            tokens::UnsignedAccessToken::new(&access_token.id).into();
        let refresh_token = tokens::RefreshToken::new(&refresh_token.id);

        Ok((refresh_token, access_token))
    }

    pub async fn validate(&self, access_token: &str) -> Result<models::Session, KeygateError> {
        let claims = self.access_token_claims(access_token).await?;

        // if the access token is unsigned, we can only trust the jti
        let Some(token) = self.storage.access_token_by_id(&claims.jwt_id).await? else {
            return Err(SessionError::NotFound.into());
        };

        if token.revoked_at.is_some() {
            return Err(JWTError::OldTokenReused.into());
        }

        if token.expires_at < chrono::Utc::now().timestamp() {
            return Err(JWTError::TokenHasExpired.into());
        }

        let Some(session) = self.storage.session_by_id(&token.id).await? else {
            return Err(SessionError::NotFound.into());
        };

        if session.revoked_at.is_some() {
            return Err(JWTError::OldTokenReused.into());
        }

        Ok(session)
    }

    pub async fn access_token_claims(
        &self,
        access_token: &str,
    ) -> Result<KeygateClaims, KeygateError> {
        let sign_jwts = {
            self.config
                .read()
                .map_err(StorageError::from)?
                .token
                .sign_jwt
        };

        if sign_jwts {
            todo!("validate signed access token")
            // SignedAccessToken::from(access_token.to_string())
            //     .verify()
            //     .map_err(KeygateError::from)?
            //     .claims
        } else {
            Ok(UnsignedAccessToken::from(access_token.to_string()).parse()?)
        }
    }

    pub async fn invalidate(&self, access_token_id: &str) -> Result<(), KeygateError> {
        Ok(self.storage.revoke_access_token(access_token_id).await?)
    }

    pub async fn all(&self, user_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn refresh_invalidate(&self, refresh_token_id: &str) -> Result<(), KeygateError> {
        Ok(self.storage.revoke_refresh_token(refresh_token_id).await?)
    }

    pub async fn refresh(
        &self,
        refresh_token_id: &str,
    ) -> Result<(tokens::AccessToken, tokens::RefreshToken), KeygateError> {
        let (refresh_expires_at, access_expires_at) = get_expiration_times(self.config.clone())?;

        let (refresh_token, access_token, _) = self
            .storage
            .refresh_token(refresh_token_id, refresh_expires_at, access_expires_at)
            .await?;

        let access_token: tokens::AccessToken =
            tokens::UnsignedAccessToken::new(&access_token.id).into();
        let refresh_token = tokens::RefreshToken::new(&refresh_token.id);

        Ok((access_token, refresh_token))
    }
}
