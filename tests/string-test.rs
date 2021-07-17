#[cfg(test)]
mod string_tests {
  use lodust;

  #[test]
  fn camel_case_test() {
    assert_eq!(lodust::camel_case("Foo Bar".to_string()), "fooBar");
    assert_eq!(lodust::camel_case("--foo-bar--".to_string()), "fooBar");
    assert_eq!(lodust::camel_case("__FOO_BAR__".to_string()), "fooBar");
  }
}
