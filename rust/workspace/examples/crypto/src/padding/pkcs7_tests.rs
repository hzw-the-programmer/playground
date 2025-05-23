use inout::block_padding::array::{
    Array,
    typenum::{U8, U10},
};
use inout::block_padding::{Padding, Pkcs7};

#[test]
fn test_1() {
    // D:\github\RustCrypto\utils\block-padding\src\lib.rs
    let msg = b"test";
    let pos = msg.len();
    let mut block: Array<u8, U8> = [0xff; 8].into();
    block[..pos].copy_from_slice(msg);
    Pkcs7::pad(&mut block, pos);
    assert_eq!(&block[..], b"test\x04\x04\x04\x04");
    let res = Pkcs7::unpad(&block).unwrap();
    assert_eq!(res, msg);
}

#[test]
fn test_2() {
    let msg = b"test";
    let pos = msg.len();
    let mut block: Array<u8, U10> = [0xff; 10].into();
    block[..pos].copy_from_slice(msg);
    Pkcs7::pad(&mut block, pos);
    assert_eq!(&block[..], b"test\x06\x06\x06\x06\x06\x06");
    let res = Pkcs7::unpad(&block).unwrap();
    assert_eq!(res, msg);
}

#[test]
#[should_panic(expected = "`pos` is bigger or equal to block size")]
fn test_3() {
    let msg = b"0123456789";
    let pos = msg.len();
    let mut block: Array<u8, U10> = [0xff; 10].into();
    block[..pos].copy_from_slice(msg);
    Pkcs7::pad(&mut block, pos);
    assert_eq!(&block[..], b"01234567890");
    let res = Pkcs7::unpad(&block).unwrap();
    assert_eq!(res, msg);
}
