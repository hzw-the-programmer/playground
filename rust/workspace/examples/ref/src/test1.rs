pub fn test() {
    let s = &[1; 5][..];
    f1(s);
    assert_eq!(s.len(), 5);

    let mut s2 = &[1; 5][..];
    f2(&mut s2);
    assert_eq!(s2.len(), 4);
}

fn f1(mut s: &[u8]) {
    s = &s[1..];
    assert_eq!(s.len(), 4);
}

fn f2(s: &mut &[u8]) {
    *s = &s[1..];
}
