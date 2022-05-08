#[test]
fn ownership() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ownership/*.rs");
}
