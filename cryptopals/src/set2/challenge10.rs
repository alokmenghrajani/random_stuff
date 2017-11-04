/**
 * Implement CBC mode
 * CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, despite
 * the fact that a block cipher natively only transforms individual blocks.
 *
 * In CBC mode, each ciphertext block is added to the next plaintext block before the next call to
 * the cipher core.
 *
 * The first plaintext block, which has no associated previous ciphertext block, is added to a
 * "fake 0th ciphertext block" called the initialization vector, or IV.
 *
 * Implement CBC mode by hand by taking the ECB function you wrote earlier, making it encrypt
 * instead of decrypt (verify this by decrypting whatever you encrypt to test), and using your XOR
 * function from the previous exercise to combine them.
 *
 * The file here is intelligible (somewhat) when CBC decrypted against "YELLOW SUBMARINE" with an
 * IV of all ASCII 0 (\x00\x00\x00 &c)
 */

use utils::base64::base64_decode;
use utils::xor::xor;

use crypto::aessafe;
use crypto::symmetriccipher::BlockEncryptor;
use crypto::symmetriccipher::BlockDecryptor;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn run() {
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

    let iv: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6];
    let key: &[u8] = "SUPER SECRET....".as_bytes();
    let plaintext = "Some long, random, string that we'll encrypt and decrypt. Magically padded \
                     to the right length..";
    let ciphertext = aes_cbc_encrypt(&iv, key, plaintext.as_bytes());
    let plaintext2 = aes_cbc_decrypt(&iv, key, &ciphertext);
    let s: String = plaintext2.into_iter().map(|x| x as char).collect();
    println!("{}", s);
    assert_eq!(s, plaintext);

    // load lines from 7.txt
    let file = File::open("./src/set2/10.txt").expect("Unable to open 10.txt");
    let lines = BufReader::new(file).lines();
    let contents: String = lines.map(|x| x.unwrap()).collect();
    let data = base64_decode(contents.as_str());
    let plaintext = aes_cbc_decrypt(&[0; 16], "YELLOW SUBMARINE".as_bytes(), &data);
    let s: String = plaintext.into_iter().map(|x| x as char).collect();
    println!("{}", s);
}

// Note: expects data to be padded
fn aes_cbc_encrypt(iv: &[u8], key: &[u8], plaintext: &[u8]) -> Vec<u8> {
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
fn aes_cbc_decrypt(iv: &[u8], key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
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

fn aes_raw_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let aes_enc = aessafe::AesSafe128Encryptor::new(key);
    let mut r = [0; 16];
    aes_enc.encrypt_block(plaintext, &mut r);
    return r.to_vec();
}

fn aes_raw_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let aes_enc = aessafe::AesSafe128Decryptor::new(key);
    let mut r = [0; 16];
    aes_enc.decrypt_block(ciphertext, &mut r);
    return r.to_vec();
}
