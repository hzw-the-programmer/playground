use super::*;
use prost::Message;

#[test]
fn test_sint32() {
    let mut msg = Msg { a: -1, b: 0 };

    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    /*
        7:0 = 0011_1000 = 38
        -1，64个1，64/7=9余1，小端: 1111_1111(总共9个) 0000_0001
    */
    assert_eq!(hex::encode(&buf), "38ffffffffffffffffff01");

    msg.a = 0;
    msg.b = -1;
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    /*
        8:0 = 0100_0000 = 40
        zigzag(-1) = 1
        (1111_1111 << 1) ^ (1111_1111 >> 7)
        1111_1110 ^ 1111_1111 = 0000_0001 = 1
    */
    assert_eq!(hex::encode(&buf), "4001");

    msg.a = 1;
    msg.b = 0;
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    assert_eq!(hex::encode(&buf), "3801");

    msg.a = 0;
    msg.b = 1;
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    /*
        (0000_0001 << 1) ^ (0000_0001 >> 7)
    */
    assert_eq!(hex::encode(&buf), "4002");
}
