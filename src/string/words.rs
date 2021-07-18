#![allow(dead_code)]

use regex::Regex;

/// (String) Split the input string into array of its words.
///
/// If the regex for filtering purpose is not provided, we will take only group of normal letters as word by default.
///
/// # Example
///
/// ```rust
/// use lodust::words;
/// use regex::Regex;
///
/// let word = words("fred, barney, & pebbles".to_string(), None);
/// // => ['fred', 'barney', 'pebbles']
///
/// let word = words("fred, barney, & pebbles".to_string(), Some(Regex::new("[^, ]+").unwrap()));
/// // => ['fred', 'barney', '&', 'pebbles']
/// ```
///

pub fn words(s: String, re: Option<Regex>) -> Vec<String> {
  match re {
    Some(rgx) => {
      return rgx
        .find_iter(&s)
        .filter_map(|w| w.as_str().parse().ok())
        .collect();
    },
    None => {
      let mut result = Vec::new();
      let mut word = String::new();
      
      for (_index, char) in s.chars().enumerate() {
        if char.is_alphanumeric() {
          word.push(char);
        } else {
          if word.len() > 0 {
            result.push(word);
            word = String::new();
          }
        }
      }
    
      if word.len() > 0 {
        result.push(word);
      }
    
      return result;
    }
  }
}
