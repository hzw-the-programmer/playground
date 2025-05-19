use super::*;

#[test]
fn test_encode() {
    assert_eq!(encode(b"MENON"), "TUVOT04=");
    assert_eq!(encode(b"geeksforgeeks"), "Z2Vla3Nmb3JnZWVrcw==");
}
