use super::map;
use super::oneof;
use super::repeated;
use prost::Message;

#[test]
fn test_repeated() {
    let t = repeated::Repeated {
        d: "hello".to_string(),
        e: vec![1, 2, 3],
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

      32
      0011 0010
      00110 010
      6:LEN

      03

      01 02 03
    */
    assert_eq!(hex::encode(&buf), "220568656c6c6f3203010203");
}

#[test]
fn test_packed() {
    let t = repeated::Packed {
        d: "hello".to_string(),
        e: vec![1, 2, 3],
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

      30
      0011 0000
      00110 000
      6:VARINT

      01

      30
      6:VARINT
      02

      30
      6:VARINT
      03
    */
    assert_eq!(hex::encode(&buf), "220568656c6c6f300130023003");
}

#[test]
fn test_oneof() {
    let t = oneof::Msg {
        f: 16,
        a: Some(oneof::msg::A::D("hello".to_string())),
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
    let t = oneof::Msg {
        f: 16,
        a: Some(oneof::msg::A::E(17)),
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

#[test]
fn test_map() {
    let mut t = map::Msg {
        g: std::collections::HashMap::new(),
    };
    t.g.insert("one".to_string(), 1);
    t.g.insert("two".to_string(), 2);

    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();

    /*
        3a
        0011 1010
        00111 010
        7:LEN

        07

        0a
        0000 1010
        00001 010
        1:LEN
        03
        6f 6e 65 // one

        10
        0001 0000
        00010 000
        2:VARINT
        01

        3a
        7:LEN
        07

        0a
        1::LEN
        03
        74 77 6f // two

        10
        2:VARINT
        02
    */

    /*
        3a 07
        0a 03 74 77 6f
        10 02

        3a 07
        0a 03 6f 6e 65
        10 01
    */
    let hex = hex::encode(&buf);
    let res1 = "3a070a036f6e6510013a070a0374776f1002";
    let res2 = "3a070a0374776f10023a070a036f6e651001";
    assert!(hex == res1 || hex == res2);

    let t2 = map::Msg2 {
        g: vec![
            map::msg2::GEntry {
                key: "one".to_string(),
                value: 1,
            },
            map::msg2::GEntry {
                key: "two".to_string(),
                value: 2,
            },
        ],
    };
    let mut buf2 = Vec::new();
    buf2.reserve(t2.encoded_len());
    t2.encode(&mut buf2).unwrap();
    assert_eq!(hex::encode(&buf2), res1);
}
