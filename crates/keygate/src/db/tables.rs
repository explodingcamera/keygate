use okv::backend::memory::MemDB;
use okv::types::serde::SerdeRmp;
use okv::{Database, Env};

use super::models;

pub(crate) type Backend = MemDB;
pub(crate) type DbEnv = Env<Backend>;

pub(crate) type Identities<DB> = Database<String, SerdeRmp<models::VersionedIdentity>, DB>;
pub(crate) type Emails<DB> = Database<String, SerdeRmp<models::VersionedEmail>, DB>;
pub(crate) type Sessions<DB> = Database<String, SerdeRmp<models::VersionedSession>, DB>;
pub(crate) type MagicLinks<DB> = Database<String, SerdeRmp<models::VersionedMagicLink>, DB>;