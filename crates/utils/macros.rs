#[macro_export]
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

pub use join_keys;
