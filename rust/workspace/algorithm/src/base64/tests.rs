use super::*;

// 往返测试（编码后再解码）
#[test]
fn test_round_trip() {
    let data = b"Hello, world! Base64 test.";
    let encoded = encode(data);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(&decoded[..], data);
}

// RFC 4648 标准测试向量
#[test]
fn test_rfc4648_vectors() {
    let cases = [
        ("", ""),
        ("f", "Zg=="),
        ("fo", "Zm8="),
        ("foo", "Zm9v"),
        ("foob", "Zm9vYg=="),
        ("fooba", "Zm9vYmE="),
        ("foobar", "Zm9vYmFy"),
    ];

    for (input, expected) in cases.iter() {
        assert_eq!(encode(input.as_bytes()), *expected);
        assert_eq!(decode(expected).unwrap(), input.as_bytes());
    }
}
