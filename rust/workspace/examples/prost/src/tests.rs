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
