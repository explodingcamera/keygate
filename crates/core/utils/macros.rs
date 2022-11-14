// https://github.com/redis-rs/redis-rs/issues/353#issuecomment-666290557
macro_rules! async_transaction {
    ($conn:expr, $keys:expr, $body:expr) => {
        loop {
            redis::cmd("WATCH").arg($keys).query_async($conn).await?;

            if let Some(response) = $body {
                redis::cmd("UNWATCH").query_async($conn).await?;
                break response;
            }
        }
    };
}

macro_rules! join_keys {
  ($($args:expr),*) => {{
    let separator = ":";

    let mut result = String::new();
    $(
      if !result.is_empty() {
        result.push_str(separator);
      }
      result.push_str($args);
    )*

    result
  }}
}

pub(crate) use async_transaction;
pub(crate) use join_keys;

mod tests {
    use crate::utils::macros::join_keys;

    fn test_join_keys() {
        assert_eq!(join_keys!("a", "b", "c"), "a:b:c");
        assert_eq!(join_keys!("a", "b", "c", "d"), "a:b:c:d");
        assert_eq!(join_keys!("a", "b", "c", "d", "e"), "a:b:c:d:e");
    }
}
