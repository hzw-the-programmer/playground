use inout::block_padding::array::{
    Array,
    typenum::{U8, U10},
};
use inout::block_padding::{Padding, ZeroPadding};

#[test]
fn test_1() {
    let msg = b"test";
    let pos = msg.len();
    let mut block: Array<u8, U8> = [0xff; 8].into();
    block[..pos].copy_from_slice(msg);
    ZeroPadding::pad(&mut block, pos);
    assert_eq!(&block[..], b"test\x00\x00\x00\x00");
    let res = ZeroPadding::unpad(&block).unwrap();
    assert_eq!(res, msg);
}

#[test]
fn test_2() {
    let msg = b"test";
    let pos = msg.len();
    let mut block: Array<u8, U10> = [0xff; 10].into();
    block[..pos].copy_from_slice(msg);
    ZeroPadding::pad(&mut block, pos);
    assert_eq!(&block[..], b"test\x00\x00\x00\x00\x00\x00");
    let res = ZeroPadding::unpad(&block).unwrap();
    assert_eq!(res, msg);
}

#[test]
fn test_3() {
    let msg = b"test\x00";
    let pos = msg.len();
    let mut block: Array<u8, U10> = [0xff; 10].into();
    block[..pos].copy_from_slice(msg);
    ZeroPadding::pad(&mut block, pos);
    assert_eq!(&block[..], b"test\x00\x00\x00\x00\x00\x00");
    let res = ZeroPadding::unpad(&block).unwrap();
    assert_eq!(res, &msg[..pos - 1]);
}
