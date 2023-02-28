use super::traits::*;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum StorageType {
    SQL,
    Redis,
}

pub trait Storage:
    StorageConfigExtension + StorageIdentityExtension + StorageProcessExtension + StorageSessionExtension
{
}
