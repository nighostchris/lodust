#![allow(dead_code)]

use regex::Regex;

/// (String) Converts the input string into lower case format.
///
/// Note that all the special characters will be removed and only valid letters remained.
///
/// # Example
///
/// ```rust
/// use lodust::kebab_case;
///
/// let kebab_cased = kebab_case("Foo Bar".to_string());
/// // => "foo bar"
///
/// let kebab_cased = kebab_case("--foo--bar--".to_string());
/// // => "foo bar"
///
/// let kebab_cased = kebab_case("__FOO_BAR__".to_string());
/// // => "foo bar"
///
/// let kebab_cased = kebab_case("fooBar".to_string());
/// // => "foo bar"
///
/// let kebab_cased = kebab_case("__fOo_-BaR__".to_string());
/// // => "f oo ba r"
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
