use crate::Keygate;

pub trait Verification: Send + Sync {}
impl Verification for Keygate {}
