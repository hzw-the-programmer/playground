use super::*;
use prost::Message;

#[test]
fn test_submsg() {
    let t = SubMsg { a: 127 };
    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();
    assert_eq!(hex::encode(&buf), "087f");

    let t = Msg { c: Some(t) };
    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();
    assert_eq!(hex::encode(&buf), "1a02087f");
}

#[test]
fn test_optional_1() {
    let t = SubMsg::decode(&hex::decode("").unwrap()[..]).unwrap();
    assert_eq!(t.a, 0);

    let t = SubMsg::decode(&hex::decode("0800").unwrap()[..]).unwrap();
    assert_eq!(t.a, 0);

    let t = SubMsg { a: 0 };
    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();
    assert_eq!(hex::encode(&buf), "");
}

#[test]
fn test_optional_2() {
    let t = SubMsg2::decode(&hex::decode("").unwrap()[..]).unwrap();
    assert_eq!(t.a, None);

    let t = SubMsg2::decode(&hex::decode("0800").unwrap()[..]).unwrap();
    assert_eq!(t.a, Some(0));

    let t = SubMsg2 { a: None };
    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();
    assert_eq!(hex::encode(&buf), "");

    let t = SubMsg2 { a: Some(0) };
    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();
    assert_eq!(hex::encode(&buf), "0800");
}
