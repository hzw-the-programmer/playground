use super::*;
use prost::Message;

#[test]
fn test_oneof() {
    let t = Msg {
        f: 16,
        a: Some(msg::A::D("hello".to_string())),
    };

    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();

    /*
        22
        0010 0010
        00100 010
        4:LEN

        05

        68 65 6c 6c 6f

        38
        0011 1000
        00111 000
        7:VARINT

        10
    */
    assert_eq!(hex::encode(&buf), "220568656c6c6f3810");
}

#[test]
fn test_oneof_2() {
    let t = Msg {
        f: 16,
        a: Some(msg::A::E(17)),
    };

    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();

    /*
        30
        0011 0000
        00110 000
        6:VARINT

        11

        38
        0011 1000
        00111 000
        7:VARINT

        10
    */
    assert_eq!(hex::encode(&buf), "30113810");
}
