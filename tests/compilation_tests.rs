#[test]
fn expected_compilation_errors_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs")
}
