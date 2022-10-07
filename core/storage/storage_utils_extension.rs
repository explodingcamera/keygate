use crate::{models, Storage};

use super::{StorageError, StorageSerdeExtension};

pub trait StorageUtilsExtension: Storage + StorageSerdeExtension + Send + Sync {
    fn create_user(&self) -> Result<(), StorageError> {
        StorageSerdeExtension::get::<models::Identity>(self, ":")?;
        Ok(())
    }
}
