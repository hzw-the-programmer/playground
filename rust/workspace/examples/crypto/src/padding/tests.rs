use inout::block_padding::array::{Array, typenum::U8};
use inout::block_padding::{Padding, Pkcs7};

#[test]
fn test_padding() {
    let msg = b"test";
    let pos = msg.len();
    let mut block: Array<u8, U8> = [0xff; 8].into();
    block[..pos].copy_from_slice(msg);
    Pkcs7::pad(&mut block, pos);
    assert_eq!(&block[..], b"test\x04\x04\x04\x04");
}
