#[async_trait::async_trait]
pub trait StorageConfigExtension: Send + Sync {
    fn config(&self) -> &crate::KeygateConfigInternal;
}
