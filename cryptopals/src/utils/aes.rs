use utils::xor::xor;
use crypto::aessafe;
use crypto::symmetriccipher::BlockEncryptor;
use crypto::symmetriccipher::BlockDecryptor;

// Note: expects data to be padded
pub fn aes_cbc_encrypt(iv: &[u8], key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    assert_eq!(iv.len(), 16);
    assert_eq!(key.len(), 16);
    assert_eq!(plaintext.len() % 16, 0);
    let mut r = Vec::with_capacity(plaintext.len());
    let mut prev: Vec<u8>;
    prev = iv.to_vec();
    for block in plaintext.chunks(16) {
        let p = xor(block, &prev);
        prev = aes_raw_encrypt(key, &p);
        r.extend(prev.iter());
    }

    return r;
}

// Note: does not unpad data
pub fn aes_cbc_decrypt(iv: &[u8], key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    assert_eq!(iv.len(), 16);
    assert_eq!(key.len(), 16);
    assert_eq!(ciphertext.len() % 16, 0);
    let mut r = Vec::with_capacity(ciphertext.len());
    let mut prev: Vec<u8>;
    prev = iv.to_vec();
    for block in ciphertext.chunks(16) {
        let p = aes_raw_decrypt(key, block);
        let plain = xor(&p, &prev);
        prev = block.to_vec();
        r.extend(plain);
    }

    return r;
}

pub fn aes_raw_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let aes_enc = aessafe::AesSafe128Encryptor::new(key);
    let mut r = [0; 16];
    aes_enc.encrypt_block(plaintext, &mut r);
    return r.to_vec();
}

pub fn aes_raw_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let aes_enc = aessafe::AesSafe128Decryptor::new(key);
    let mut r = [0; 16];
    aes_enc.decrypt_block(ciphertext, &mut r);
    return r.to_vec();
}
