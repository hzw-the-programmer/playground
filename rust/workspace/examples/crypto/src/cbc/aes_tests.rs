use aes::cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyIvInit};
use hex_literal::hex;
use inout::block_padding::Pkcs7;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// D:\github\RustCrypto\block-modes\cbc\src\lib.rs
#[test]
fn test_1() {
    let key = [0x42; 16];
    let iv = [0x24; 16];
    // len = 34
    let plaintext = *b"hello world! this is my plaintext.";
    // len = 48 = 34 + 14 = 16 + 16 + 2 + 14
    let ciphertext = hex! {
        "c7fe247ef97b21f07cbdd26cb5d346bf"
        "d27867cb00d9486723e159978fb9a5f9"
        "14cfb228a710de4171e396e7b6cf859e"
    };

    let mut buf = [0u8; 48];
    let pt_len = plaintext.len();
    buf[..pt_len].copy_from_slice(&plaintext);
    let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    assert_eq!(ct, &ciphertext[..]);

    let pt = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded::<Pkcs7>(&mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext[..]);

    let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_b2b::<Pkcs7>(&plaintext, &mut buf)
        .unwrap();
    assert_eq!(ct, &ciphertext[..]);

    let mut buf = [0u8; 48];
    let pt = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b::<Pkcs7>(ct, &mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext[..]);
}

#[test]
fn test_2() {
    let key = [0x42; 16];
    let iv = [0x24; 16];
    let plaintext = b"0123456789123456";

    let mut buf = [0u8; 16];
    let ct =
        Aes128CbcEnc::new(&key.into(), &iv.into()).encrypt_padded_b2b::<Pkcs7>(plaintext, &mut buf);
    assert!(ct.is_err());

    // let mut buf = [0u8; 31];
    let mut buf = [0u8; 32];
    // let mut buf = [0u8; 33];
    let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_b2b::<Pkcs7>(plaintext, &mut buf)
        .unwrap();
    // let mut buf = [0u8; 31];
    let mut buf = [0u8; 32];
    // let mut buf = [0u8; 33];
    let pt = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b::<Pkcs7>(ct, &mut buf)
        .unwrap();
    assert_eq!(pt, plaintext);
}
