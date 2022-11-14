mod ac_extension;
mod identity_extension;
mod process_extension;
mod redis_extension;
mod serde_extension;
mod session_extension;

// storage backends can implement this trait instead of implementing the Storage Extension traits
// directly to get default implementations for the extension traits
pub trait GenericKV: BaseStorage + StorageSerdeExtension + Send + Sync {}

pub use ac_extension::StorageAccessControlExtension;
pub use identity_extension::StorageIdentityExtension;
pub use process_extension::StorageProcessExtension;
pub use serde_extension::StorageSerdeExtension;
pub use session_extension::StorageSessionExtension;

pub use redis_extension::RedisExtensions;

use super::BaseStorage;
