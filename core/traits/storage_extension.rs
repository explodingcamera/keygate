use crate::{storage::StorageError, KeySignal};

pub trait StorageExtension {
    fn get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned;
    fn set<T>(&self, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize;

    fn pget<T>(&self, prefix: &str, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned;
    fn pset<T>(&self, prefix: &str, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize;
}
impl StorageExtension for KeySignal {
    fn get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self.storage.get_u8(key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn set<T>(&self, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize,
    {
        let val = rmp_serde::to_vec(value)?;
        self.storage.set_u8(key, &val)
    }

    fn pget<T>(&self, prefix: &str, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self.storage.pget_u8(prefix, key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn pset<T>(&self, prefix: &str, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize,
    {
        let val = rmp_serde::to_vec(value)?;
        self.storage.pset_u8(prefix, key, &val)
    }
}
