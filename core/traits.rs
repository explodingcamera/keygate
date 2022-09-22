mod identity;
mod login;
mod metadata;
mod oauth;
mod recovery;
mod registration;
mod session;
mod signup;
mod verification;

pub mod all {
    pub use {
        super::identity::Identity, super::login::Login, super::metadata::Metadata,
        super::oauth::OAuth, super::recovery::Recovery, super::registration::Registration,
        super::session::Session, super::signup::Signup, super::verification::Verification,
    };
}

pub mod storage_extension;
