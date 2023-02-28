pub const IDENTITY_BY_ID: &str = "kg:id:id"; // maps id to identity
pub const IDENTITY_USERNAME_INDEX: &str = "kg:id:i_un"; // a list of all usernames
pub const IDENTITY_ID_BY_USERNAME: &str = "kg:id:by_un"; // maps username to identity id
pub const IDENTITY_ID_BY_EMAIL: &str = "kg:id:by_em"; // maps email to identity id
pub const IDENTITY_SESSIONS: &str = "kg:id:sn"; // maps identity id to a list of session ids

pub const SESSION_BY_ID: &str = "kg:sn:id"; // maps session id to session
pub const REFRESH_TOKEN_BY_ID: &str = "kg:sn:rt"; // maps refresh token id to refresh token

pub const PROCESS_BY_ID: &str = "kg:pr:id"; // maps process id to process
pub const PROCESS_ID_BY_TOKEN: &str = "kg:pr:by_tk"; // maps tokens (e.g otp + device_id, magic link token) to process id

pub const ACTOR_ROLES: &str = "kg:ac:roles"; // maps actor id to a list of roles
pub const RESOURCE_ACTORS_BY_ROLE: &str = "kg:ac:res_acs"; // maps resource id to a list of actor ids
