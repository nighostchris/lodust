#![allow(dead_code)]

/// (String) Converts first character of the input string to upper case and the remaining characters to lower case.
///
/// # Example
///
/// ```rust
/// use lodust::capitalize;
///
/// let capitalized = capitalize("Foo Bar".to_string());
/// // => "Foo bar"
///
/// let capitalized = capitalize("--foo--bar--".to_string());
/// // => "--foo--bar--"
///
/// let capitalized = capitalize("__FOO_BAR__".to_string());
/// // => "__foo_bar__"
/// ```
///

pub fn capitalize(s: String) -> String {
  let mut result = String::new();

  for (index, char) in s.chars().enumerate() {
    if index == 0 {
      result.push(char.to_ascii_uppercase());
    } else {
      result.push(char.to_ascii_lowercase());
    }
  }

  return result;
}
