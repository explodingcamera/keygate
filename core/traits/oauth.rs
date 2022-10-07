use crate::Keygate;

pub trait OAuth: Send + Sync {}

impl OAuth for Keygate {}
