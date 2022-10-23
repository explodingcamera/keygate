macro_rules! join_keys {
  ($($args:expr),*) => {{
      let result = String::new();
      $(
          let result = result + ":" + $args;
      )*
      result
  }}
}

pub(crate) use join_keys;

pub const PREFIX: &str = "kg";
pub const IDENTITY_PREFIX: &str = "kg:id";
pub const SESSION_PREFIX: &str = "kg:sn";

pub const IDENTITY_BY_ID: &str = "kg:id:id"; // maps id to identity
pub const IDENTITY_USERNAME_INDEX: &str = "kg:id:i_un"; // a list of all usernames
pub const IDENTITY_ID_BY_USERNAME: &str = "kg:id:by_un"; // maps username to identity id
pub const IDENTITY_ID_BY_EMAIL: &str = "kg:id:by_em"; // maps email to identity id
