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
pub const IDENTITY_SESSIONS: &str = "kg:id:sn"; // maps identity id to a list of session ids

pub const SESSION_BY_ID: &str = "kg:sn:id"; // maps session id to session
pub const REFRESH_TOKEN_BY_ID: &str = "kg:sn:rt"; // maps refresh token id to refresh token
pub const ACCESS_TOKEN_BY_ID: &str = "kg:sn:at"; // maps access token id to access token

pub const FLOW_BY_ID: &str = "kg:fl:id"; // maps flow id to flow
pub const FLOWTOKEN_BY_ID: &str = "kg:fl:by_tk"; // maps tokens (e.g otp + device_id, magic link token) to FLOWTOKENs
