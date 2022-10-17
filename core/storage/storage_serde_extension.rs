use crate::{storage::StorageError, Storage};

pub trait StorageSerdeExtension: Storage {
    fn _get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self._get_u8(key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn _set<T>(&self, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize + ?Sized,
    {
        let val = rmp_serde::to_vec(value)?;
        self._set_u8(key, &val)
    }

    fn _pget<T>(&self, prefix: &str, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self._pget_u8(prefix, key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn _pset<T>(&self, prefix: &str, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize + ?Sized,
    {
        let val = rmp_serde::to_vec(value)?;
        self._pset_u8(prefix, key, &val)
    }

    fn to_bytes(value: &impl serde::Serialize) -> Result<Vec<u8>, StorageError> {
        Ok(rmp_serde::to_vec(value)?)
    }
    fn from_bytes<T>(bytes: &[u8]) -> Result<T, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(rmp_serde::from_slice(bytes)?)
    }
}

impl StorageSerdeExtension for dyn Storage + Send + Sync {
    fn _get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self._get_u8(key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn _set<T>(&self, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize + ?Sized,
    {
        let val = rmp_serde::to_vec(value)?;
        self._set_u8(key, &val)
    }

    fn _pget<T>(&self, prefix: &str, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self._pget_u8(prefix, key)?;

        if let Some(data) = bytes {
            if data.is_empty() {
                return Ok(None);
            }

            let res = rmp_serde::from_slice(data.as_slice())?;
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn _pset<T>(&self, prefix: &str, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize + ?Sized,
    {
        let val = rmp_serde::to_vec(value)?;
        self._pset_u8(prefix, key, &val)
    }
}
