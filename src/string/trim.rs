#![allow(dead_code)]

/// (String) Removes leading and trailing whitespace from string
///
/// Note that all the special characters will be removed and only valid letters remained.
///
/// # Example
///
/// ```rust
/// use lodust::kebab_case;
///
/// let kebab_cased = kebab_case("  Foo Bar ".to_string());
/// // => "Foo Bar"
///
/// ```
///

pub fn trim(s: String, trim_string: String) -> String {
    if trim_string.len() > s.len() {
        return s;
    }

    let mut start_index = 0;
    let mut end_index = s.len();

    let string_end = &s[s.len() - trim_string.len()..];
    if string_end == trim_string {
        end_index = s.len() - trim_string.len();
    }

    let string_start = &s[..trim_string.len()];
    if string_start == trim_string {
        start_index = trim_string.len();
    }

    let trimmed_str_slice = &s[start_index..end_index];

    let result = trimmed_str_slice.to_string();

    if (&result[result.len() - trim_string.len()..]) == trim_string {
        return trim(result, trim_string);
    }

    return result;
}