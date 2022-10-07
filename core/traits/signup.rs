use crate::Keygate;

pub trait Signup: Send + Sync {}
impl Signup for Keygate {}
