use super::test4;
use prost::Message;

#[test]
fn test_test4() {
    let t = test4::Test4 {
        d: "hello".to_string(),
        e: vec![1, 2, 3],
    };

    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();

    assert_eq!(hex::encode(&buf), "220568656c6c6f3203010203");
}
