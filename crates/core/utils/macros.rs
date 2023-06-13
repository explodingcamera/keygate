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

pub(crate) use join_keys;

mod tests {
    use crate::utils::macros::join_keys;

    fn test_join_keys() {
        assert_eq!(join_keys!("a", "b", "c"), "a:b:c");
        assert_eq!(join_keys!("a", "b", "c", "d"), "a:b:c:d");
        assert_eq!(join_keys!("a", "b", "c", "d", "e"), "a:b:c:d:e");
    }
}
