use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use hex_literal::hex;

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

#[test]
fn test1() {
    let key = [0x42; 16];
    let iv = [0x24; 16];
    let plaintext = *b"hello world! this is my plaintext.";
    let ciphertext = hex!("3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4");

    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());

    let mut buf = plaintext.to_vec();
    cipher.apply_keystream(&mut buf);
    assert_eq!(buf, ciphertext);

    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
    for chunk in buf.chunks_mut(3) {
        cipher.apply_keystream(chunk);
    }
    assert_eq!(buf, plaintext);
}

#[test]
fn test2() {
    let key = [0x42; 16];
    let iv = [0x24; 16];
    let plaintext = *b"hello world! this is my plaintext.";
    let ciphertext = hex!("3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4");

    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());

    let mut buf = plaintext.to_vec();
    cipher.apply_keystream(&mut buf);
    assert_eq!(buf, ciphertext);

    cipher.seek(0);
    for chunk in buf.chunks_mut(3) {
        cipher.apply_keystream(chunk);
    }
    assert_eq!(buf, plaintext);
}

#[test]
fn test3() {
    let key = [0x42; 16];
    let iv = [0x24; 16];
    let plaintext = *b"hello world! this is my plaintext.";
    let ciphertext = hex!("3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4");

    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());

    let mut buf1 = [0; 34];
    cipher.apply_keystream_b2b(&plaintext, &mut buf1).unwrap();
    assert_eq!(buf1, ciphertext);

    let mut buf2 = [0; 34];
    cipher.seek(0);
    cipher.apply_keystream_b2b(&buf1, &mut buf2).unwrap();
    assert_eq!(buf2, plaintext);
}
