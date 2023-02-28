type ResourceType<'a> = &'a str;
type ResourceId<'a> = &'a str;
type Resource<'a> = (ResourceType<'a>, ResourceId<'a>);
use crate::storage::StorageError;

#[async_trait::async_trait]
pub trait StorageAccessControlExtension: Send + Sync {
    async fn ac_allowed(&self, actor: &str, resource: Resource, action: &str) -> Result<bool, StorageError>;
    async fn ac_roles(&self, actor: &str) -> Result<Vec<String>, StorageError>;
    async fn ac_actors(&self, resource: Resource, role: &str) -> Result<Vec<String>, StorageError>;
    async fn ac_add_role(&self, actor: &str, resource: Resource, role: &str) -> Result<(), StorageError>;
    async fn ac_del_role(&self, actor: &str, resource: Resource, role: &str) -> Result<(), StorageError>;
}
