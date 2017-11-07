use utils::xor::xor;
use utils::pkcs7::pkcs7_pad;
use crypto::aessafe;
use crypto::symmetriccipher::BlockEncryptor;
use crypto::symmetriccipher::BlockDecryptor;

// CBC encryption:
//      plain 0     plain 1     ...
//         |           |
//         v           v
// iv --> xor   ----> xor   --> ...
//         |   |       |   |
//         v   |       v   |
//   K -> AES  | K -> AES  |
//         |   |       |   |
//         |---        |---
//         v           v
//       enc 0       enc 1      ...
pub fn aes_cbc_encrypt(iv: &[u8], key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    assert_eq!(iv.len(), 16);
    assert_eq!(key.len(), 16);
    let mut r = Vec::with_capacity(plaintext.len());
    let mut prev: Vec<u8>;
    prev = iv.to_vec();
    for block in pkcs7_pad(plaintext, 16).chunks(16) {
        let p = xor(block, &prev);
        prev = aes_raw_encrypt(key, &p);
        r.extend(prev.iter());
    }

    return r;
}

// CBC decryption:
//       enc 0       enc 1      ...
//         |---        |---
//         |   |       |   |
//         v   |       v   |
//   K -> AES  | K -> AES  |
//         |   |       |   |
//         v   |       v   |
// iv --> xor   ----> xor   --> ...
//         |           |
//         |           |
//         v           v
//      plain 0     plain 1     ...

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

pub fn aes_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut r = Vec::with_capacity(plaintext.len());
    for block in pkcs7_pad(plaintext, 16).chunks(16) {
        r.extend(aes_raw_encrypt(key, block));
    }
    return r;
}

// Note: does not unpad data
pub fn aes_ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), 16);
    assert_eq!(ciphertext.len() % 16, 0);
    let mut r = Vec::with_capacity(ciphertext.len());
    for block in ciphertext.chunks(16) {
        let p = aes_raw_decrypt(key, block);
        r.extend(p);
    }
    return r;
}

pub fn aes_raw_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), 16);
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
