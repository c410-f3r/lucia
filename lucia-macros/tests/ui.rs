#[test]
fn ui() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/ui/error_enum/*.rs");
}
