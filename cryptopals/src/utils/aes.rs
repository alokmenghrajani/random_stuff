use utils::pkcs7::pkcs7_pad;
use utils::xor::xor;

use byteorder::{ByteOrder, LittleEndian};
use crypto::aessafe;
use crypto::symmetriccipher::BlockDecryptor;
use crypto::symmetriccipher::BlockEncryptor;

// CTR encryption/decryption:
//           nonce + counter      nonce + counter   ...
//                 |                    |
//                 v                    v
//           K -> AES             K -> AES          ...
//                 |                    |
//                 v                    v
//  plain/enc --> xor    plain/enc --> xor          ...
//                 |                    |
//                 v                    v
//              enc/plain            enc/plain      ...
pub fn aes_ctr(key: &[u8], nonce: &[u8], input: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), 16);
    assert_eq!(nonce.len(), 8);

    let mut num_blocks = (input.len() / 16) as u64;
    if input.len() % 16 != 0 {
        num_blocks += 1;
    }

    // create the stream. TODO: make this a real stream?
    let mut s = Vec::with_capacity(num_blocks as usize * 16);
    for i in 0..(num_blocks as u64) {
        let mut buf = [0; 16];
        for j in 0..8 {
            buf[j] = nonce[j];
        }
        LittleEndian::write_u64(buf.get_mut(8..16).unwrap(), i);
        s.extend(aes_raw_encrypt(&key, &buf));
    }

    let mut r = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        r.push(s[i as usize] ^ input[i as usize]);
    }
    return r;
}

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
    assert_eq!(plaintext.len(), 16);

    let aes_enc = aessafe::AesSafe128Encryptor::new(key);
    let mut r = [0; 16];
    aes_enc.encrypt_block(plaintext, &mut r);
    return r.to_vec();
}

pub fn aes_raw_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), 16);
    assert_eq!(ciphertext.len(), 16);

    let aes_enc = aessafe::AesSafe128Decryptor::new(key);
    let mut r = [0; 16];
    aes_enc.decrypt_block(ciphertext, &mut r);
    return r.to_vec();
}
