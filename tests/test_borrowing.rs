#[test]
fn borrowed_views_cannot_outlive_or_alias_mutation() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
}
