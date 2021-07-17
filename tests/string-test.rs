#[cfg(test)]
mod string_tests {
  use lodust::string;

  #[test]
  fn camel_case_test() {
    assert_eq!(string::camel_case("Foo Bar".to_string()), "fooBar");
    assert_eq!(string::camel_case("--foo-bar--".to_string()), "fooBar");
    assert_eq!(string::camel_case("__FOO_BAR__".to_string()), "fooBar");
  }
}
