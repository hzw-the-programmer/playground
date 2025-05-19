use super::*;

#[test]
fn test_encode() {
    assert_eq!(encode(b"MENON"), "TUVOT04=");
    assert_eq!(encode(b"geeksforgeeks"), "Z2Vla3Nmb3JnZWVrcw==");
}

#[test]
fn test_encode_empty() {
    assert_eq!(encode(b""), "");
}

#[test]
fn test_encode_three_bytes_no_padding() {
    assert_eq!(encode(b"Man"), "TWFu");
}

#[test]
fn test_encode_one_byte_two_pads() {
    assert_eq!(encode(b"A"), "QQ==");
}

#[test]
fn test_encode_two_bytes_one_pad() {
    assert_eq!(encode(b"AB"), "QUI=");
}
