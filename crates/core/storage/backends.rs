use super::traits::*;

#[derive(Clone, Copy, Debug)]
pub enum StorageType {
    SQL,
    Redis,
}

pub trait Storage:
    StorageConfigExtension + StorageIdentityExtension + StorageProcessExtension + StorageSessionExtension
{
}
