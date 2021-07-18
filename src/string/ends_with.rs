#![allow(dead_code)]

/// (String) Checks if string ends with the given target string.
///
/// If the position to search up to is not provided, we will search through the whole string by default.
///
/// # Example
///
/// ```rust
/// use lodust::ends_with;
///
/// let is_ends_with = ends_with("abc".to_string(), "c".to_string(), None);
/// // => true
///
/// let is_ends_with = ends_with("abc".to_string(), "b".to_string(), None);
/// // => false
///
/// let is_ends_with = ends_with("abc".to_string(), "bc".to_string(), Some(2));
/// // => false
/// ```
///

pub fn ends_with(s: String, target: String, position: Option<i32>) -> bool {
  let unwrapped_position = position.unwrap_or(-1);

  if unwrapped_position >= 0 {
    let sliced = &s[0..unwrapped_position as usize];
    return sliced.ends_with(target.as_str());
  } else {
    return s.ends_with(target.as_str());
  }
}
