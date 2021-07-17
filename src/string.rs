#![allow(dead_code)]
pub fn camel_case(s: String) -> String {
  let mut result = String::new();
  let mut is_last_special_char = false;

  for (_index, char) in s.chars().enumerate() {
    if char.is_alphanumeric() {
      if is_last_special_char {
        if result.len() > 0 {
          result.push(char.to_ascii_uppercase());
        } else {
          result.push(char.to_ascii_lowercase());
        }

        is_last_special_char = false;
      } else {
        result.push(char.to_ascii_lowercase());
      }
    } else {
      is_last_special_char = true;
    }
  }

  return result;
}
