use aes::Aes128;
use aes::cipher::{Array, BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};

#[test]
fn test_aes() {
    // D:\github\RustCrypto\block-ciphers\aes\src\lib.rs
    let key = Array::from([0u8; 16]);
    let mut block = Array::from([42u8; 16]);

    let cipher = Aes128::new(&key);

    let block_copy = block.clone();

    cipher.encrypt_block(&mut block);
    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);

    cipher.decrypt_block(&mut block);
    cipher.encrypt_block(&mut block);
    assert_eq!(block, block_copy);

    let mut blocks = [block; 100];
    cipher.encrypt_blocks(&mut blocks);
    for block in blocks.iter_mut() {
        cipher.decrypt_block(block);
        assert_eq!(block, &block_copy);
    }

    cipher.decrypt_blocks(&mut blocks);
    for block in blocks.iter_mut() {
        cipher.encrypt_block(block);
        assert_eq!(block, &block_copy);
    }
}

#[test]
fn test_2() {
    let key = Array::from([0u8; 16]);
    let mut block = Array::from([42u8; 16]);
    let block_copy = block.clone();

    let cipher = Aes128::new(&key);

    cipher.encrypt_block(&mut block);
    assert_ne!(block, block_copy);

    cipher.encrypt_block(&mut block);
    assert_ne!(block, block_copy);

    cipher.decrypt_block(&mut block);
    assert_ne!(block, block_copy);

    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);
}
