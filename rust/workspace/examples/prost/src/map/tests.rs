use super::*;
use prost::Message;

#[test]
fn test_map() {
    let mut t = Msg {
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

    let t2 = Msg2 {
        g: vec![
            msg2::GEntry {
                key: "one".to_string(),
                value: 1,
            },
            msg2::GEntry {
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
