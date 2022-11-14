use crate::{
    storage::{BaseStorage, StorageError},
    utils::serialize,
};

pub async fn deserialize<T>(bytes: Option<Vec<u8>>) -> Result<Option<T>, StorageError>
where
    T: serde::de::DeserializeOwned,
{
    let Some(data) = bytes else {
       return Ok(None)
    };

    if data.is_empty() {
        return Ok(None);
    }

    let res = serialize::from_bytes(data.as_slice())?;
    Ok(Some(res))
}

#[async_trait::async_trait]
pub trait StorageSerdeExtension: BaseStorage + Sync {
    async fn _get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: serde::de::DeserializeOwned,
        Self: Sized,
    {
        let bytes = self._get_u8(key).await?;
        deserialize(bytes).await
    }

    async fn _set<T>(&self, key: &str, value: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize + ?Sized + Sync,
        Self: Sized,
    {
        let val = serialize::to_bytes(&value)?;
        self._set_u8(key, &val).await
    }
}
