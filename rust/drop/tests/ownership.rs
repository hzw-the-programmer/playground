#[test]
fn ownership() {
    let t = trybuild::TestCases::new();
    t.compile_fail("fail\\vec-push.rs");
}
