use super::StorageSerdeExtension;

type ResourceType<'a> = &'a str;
type ResourceId<'a> = &'a str;
type Resource<'a> = (ResourceType<'a>, ResourceId<'a>);
use crate::storage::{BaseStorage, StorageError};

#[async_trait::async_trait]
pub trait StorageAccessControlExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    async fn is_allowed(&self, actor: &str, resource: Resource, action: &str) -> Result<bool, StorageError>;

    async fn get_roles(&self, actor: &str) -> Result<Vec<String>, StorageError>;

    async fn get_actors(&self, resource: Resource, role: &str) -> Result<Vec<String>, StorageError>;

    async fn add_role(&self, actor: &str, resource: Resource, role: &str) -> Result<(), StorageError>;

    async fn remove_role(&self, actor: &str, resource: Resource, role: &str) -> Result<(), StorageError>;
}
