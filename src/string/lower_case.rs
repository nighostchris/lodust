#![allow(dead_code)]

use regex::Regex;

/// (String) Converts the input string into lower case format.
///
/// # Example
///
/// ```rust
/// use lodust::lower_case;
///
/// let kebab_cased = lower_case("Foo Bar".to_string());
/// // => "foo bar"
///
/// let kebab_cased = lower_case("--foo--bar--".to_string());
/// // => "foo bar"
///
/// let kebab_cased = lower_case("__FOO_BAR__".to_string());
/// // => "foo bar"
///
/// let kebab_cased = lower_case("fooBar".to_string());
/// // => "foo bar"
///
/// let kebab_cased = lower_case("__fOo_-BaR__".to_string());
/// // => "foo bar"
/// ```
///

pub fn lower_case(s: String) -> String {
    let mut result: String = s.chars()
        .map(|x| {
            match x {
                '-' => ' ',
                '_' => ' ',
                _ => x,
            }
        }).collect();

    let re = Regex::new(r"\s+").unwrap();
    result = re.replace_all(&result, " ").to_string();

    result = result.to_lowercase();

    let str = &result[..];
    result = str.trim().to_string();

    return result;
}
