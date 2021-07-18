#![allow(dead_code)]

/// (String) Converts the input string into Kebab-Case format.
///
/// Note that all the special characters will be removed and only valid letters remained.
///
/// # Example
///
/// ```rust
/// use lodust::kebab_case;
///
/// let kebab_cased = kebab_case("Foo Bar".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("--foo--bar--".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("__FOO_BAR__".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("fooBar".to_string());
/// // => "foo-bar"
///
/// let kebab_cased = kebab_case("__fOo_-BaR__".to_string());
/// // => "f-oo-ba-r"
/// ```
///

pub fn kebab_case(s: String) -> String {
  let mut result = String::new();
  let mut is_last_special_char = false;
  let mut is_last_small_char = false;

  for (_index, char) in s.chars().enumerate() {
    if char.is_alphanumeric() {
      if is_last_special_char {
        if result.len() > 0 {
          result.push('-');
        }

        is_last_special_char = false;
      } else if is_last_small_char && char.is_uppercase() && result.len() > 0 {
        result.push('-');
      }

      result.push(char.to_ascii_lowercase());

      if char.is_lowercase() {
        is_last_small_char = true;
      } else {
        is_last_small_char = false;
      }
    } else {
      is_last_special_char = true;
    }
  }

  return result;
}
