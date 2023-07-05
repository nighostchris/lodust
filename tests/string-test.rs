#[cfg(test)]
mod string_tests {
  use lodust;
  use regex::Regex;

  macro_rules! string_vec {
    ($($x:expr), *) => (vec![$($x.to_string()), *]);
  }

  #[test]
  fn camel_case_test() {
    assert_eq!(lodust::camel_case("Foo Bar".to_string()), "fooBar");
    assert_eq!(lodust::camel_case("--foo--bar--".to_string()), "fooBar");
    assert_eq!(lodust::camel_case("__FOO_BAR__".to_string()), "fooBar");
  }

  #[test]
  fn capitalize_test() {
    assert_eq!(lodust::capitalize("Foo Bar".to_string()), "Foo bar");
    assert_eq!(lodust::capitalize("--foo--bar--".to_string()), "--foo--bar--");
    assert_eq!(lodust::capitalize("__FOO_BAR__".to_string()), "__foo_bar__");
  }

  #[test]
  fn ends_with_test() {
    assert_eq!(lodust::ends_with("abc".to_string(), "c".to_string(), None), true);
    assert_eq!(lodust::ends_with("abc".to_string(), "b".to_string(), None), false);
    assert_eq!(lodust::ends_with("abc".to_string(), "b".to_string(), Some(2)), true);
    assert_eq!(lodust::ends_with("abc".to_string(), "bc".to_string(), Some(1)), false);
    assert_eq!(lodust::ends_with("abc".to_string(), "bc".to_string(), Some(2)), false);
    assert_eq!(lodust::ends_with("abc".to_string(), "bc".to_string(), Some(3)), true);
    assert_eq!(lodust::ends_with("abc".to_string(), "bc".to_string(), None), true);
  }

  #[test]
  fn lower_case_test() {
    assert_eq!(lodust::lower_case("Foo Bar".to_string()), "foo bar");
    assert_eq!(lodust::lower_case("--foo--bar--".to_string()), "foo bar");
    assert_eq!(lodust::lower_case("fooBar".to_string()), "foobar");
    assert_eq!(lodust::lower_case("__FOO_BAR__".to_string()), "foo bar");
    assert_eq!(lodust::lower_case("__fOo_-BaR__".to_string()), "foo bar");
  }

  #[test]
  fn kebab_case_test() {
    assert_eq!(lodust::kebab_case("Foo Bar".to_string()), "foo-bar");
    assert_eq!(lodust::kebab_case("--foo--bar--".to_string()), "foo-bar");
    assert_eq!(lodust::kebab_case("fooBar".to_string()), "foo-bar");
    assert_eq!(lodust::kebab_case("__FOO_BAR__".to_string()), "foo-bar");
    assert_eq!(lodust::kebab_case("__fOo_-BaR__".to_string()), "f-oo-ba-r");
  }

  #[test]
  fn words_test() {
    assert_eq!(lodust::words("fred, barney, & pebbles".to_string(), None), string_vec!["fred", "barney", "pebbles"]);
    assert_eq!(
      lodust::words("fred, barney, & pebbles".to_string(), Some(Regex::new("[^, ]+").unwrap())),
      string_vec!["fred", "barney", "&", "pebbles"],
    );
  }
}
