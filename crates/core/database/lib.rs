pub mod identity;
pub use identity::Entity as Identity;

pub trait ToProto<T> {
    fn to_proto_public(&self) -> T;
    fn to_proto_private(&self) -> T;
}
