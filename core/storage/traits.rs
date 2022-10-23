mod identity_extension;
mod serde_extension;

// storage backends can implement this trait instead of implementing the Storage Extension traits
// directly to get default implementations for the extension traits
pub trait GenericKV: BaseStorage + StorageSerdeExtension + Send + Sync {}

pub use identity_extension::StorageIdentityExtension;
pub use serde_extension::StorageSerdeExtension;

use super::BaseStorage;
