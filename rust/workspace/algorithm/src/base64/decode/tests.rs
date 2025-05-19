use super::*;

#[test]
fn test_decode_empty() {
    assert_eq!(decode("").unwrap(), b"");
}

#[test]
fn test_decode_no_padding() {
    assert_eq!(decode("TWFu").unwrap(), b"Man");
}

#[test]
fn test_decode_two_pads() {
    assert_eq!(decode("QQ==").unwrap(), b"A");
}

#[test]
fn test_decode_one_pad() {
    assert_eq!(decode("QUI=").unwrap(), b"AB");
}

// 错误处理测试
#[test]
fn test_decode_invalid_length() {
    assert!(decode("Q").is_err()); // 长度 1，非 4 的倍数
}

#[test]
fn test_decode_invalid_char() {
    assert!(decode("T!WF").is_err()); // 包含非法字符 '!'
}

#[test]
fn test_decode_pad_in_wrong_position() {
    assert!(decode("TQ=Q").is_err()); // 填充符不在末尾
}

#[test]
fn test_decode_non_canonical_padding() {
    assert!(decode("TWFu===").is_err()); // 多余填充符
}
