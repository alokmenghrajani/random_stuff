/**
 * The CBC padding oracle
 * This is the best-known attack on modern block-cipher cryptography.
 *
 * Combine your padding code and your CBC code to write two functions.
 *
 * The first function should select at random one of the following 10 strings:
 *
 * MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=
 * MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=
 * MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==
 * MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==
 * MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl
 * MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==
 * MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==
 * MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=
 * MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=
 * MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93
 * ... generate a random AES key (which it should save for all future encryptions), pad the string
 * out to the 16-byte AES block size and CBC-encrypt it under that key, providing the caller the
 * ciphertext and IV.
 *
 * The second function should consume the ciphertext produced by the first function, decrypt it,
 * check its padding, and return true or false depending on whether the padding is valid.
 *
 * What you're doing here.
 * This pair of functions approximates AES-CBC encryption as its deployed serverside in web
 * applications; the second function models the server's consumption of an encrypted session token,
 * as if it was a cookie.
 *
 * It turns out that it's possible to decrypt the ciphertexts provided by the first function.
 *
 * The decryption here depends on a side-channel leak by the decryption function. The leak is the
 * error message that the padding is valid or not.
 *
 * You can find 100 web pages on how this attack works, so I won't re-explain it. What I'll say is
 * this:
 *
 * The fundamental insight behind this attack is that the byte 01h is valid padding, and occur in
 * 1/256 trials of "randomized" plaintexts produced by decrypting a tampered ciphertext.
 *
 * 02h in isolation is not valid padding.
 *
 * 02h 02h is valid padding, but is much less likely to occur randomly than 01h.
 *
 * 03h 03h 03h is even less likely.
 *
 * So you can assume that if you corrupt a decryption AND it had valid padding, you know what that
 * padding byte is.
 *
 * It is easy to get tripped up on the fact that CBC plaintexts are "padded". Padding oracles have
 * nothing to do with the actual padding on a CBC plaintext. It's an attack that targets a specific
 * bit of code that handles decryption. You can mount a padding oracle on any CBC block, whether
 * it's padded or not.
 */

use utils::aes::aes_cbc_decrypt;
use utils::aes::aes_cbc_encrypt;
use utils::base64::base64_decode;
use utils::hex::hex_decode;
use utils::pkcs7::pkcs7_unpad;

use rand::Rng;
use rand;
use std::io::prelude::*;
use std::io;

pub fn run() {
    let (iv, ciphertext) = encrypt();

    // perform a padding oracle attack on each block
    let mut blocks: Vec<&[u8]> = Vec::new();
    blocks.push(&iv);
    blocks.extend(ciphertext.chunks(16));
    let mut r = String::new();
    print!("cracking ");
    for i in 1..blocks.len() {
        let t = padding_oracle_attack(blocks[i - 1], blocks[i]);
        for j in (0..16).rev() {
            r.push((blocks[i - 1][15 - j as usize] ^ t[15 - j as usize] ^ (j + 1)) as char);
        }
        print!(".");
        io::stdout().flush().ok();
    }
    println!("");
    println!("solution:  {}", r);
}

fn padding_oracle_attack(iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    return match padding_oracle_attack_r(iv, ciphertext, 0) {
        Some(v) => v,
        None => {
            println!("something went wrong...");
            Vec::new()
        }
    };
}

fn padding_oracle_attack_r(iv: &[u8], ciphertext: &[u8], offset: u8) -> Option<Vec<u8>> {
    // fix padding from 0x01 to 0x02, from 0x02 to 0x03, etc.
    let mut iv = iv.to_vec();
    for j in (15 - offset + 1)..16 {
        iv[j as usize] = iv[j as usize] ^ (offset) ^ (offset + 1)
    }

    for i in 0..256 {
        iv[15 - offset as usize] = i as u8;
        if padding_oracle(&iv, &ciphertext) {
            // println!("found byte! offset: {}, byte: {}", offset, i);
            if offset == 15 {
                return Some(vec![i as u8]);
            } else {
                match padding_oracle_attack_r(&iv, ciphertext, offset + 1) {
                    Some(v) => {
                        let mut v = v.clone();
                        v.push(i as u8);
                        return Some(v);
                    }
                    None => {}
                };
            }
        }
    }
    return None;
}

fn encrypt() -> (Vec<u8>, Vec<u8>) {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let inputs = vec!["MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
                      "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
                      "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
                      "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
                      "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
                      "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
                      "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
                      "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
                      "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
                      "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"];
    let mut rng = rand::thread_rng();
    let input = base64_decode(inputs[(rng.next_u32() % inputs.len() as u32) as usize]);
    println!("plaintext: {}", String::from_utf8(input.clone()).unwrap());
    let mut iv = Vec::new();
    for _ in 0..16 {
        iv.push(rng.gen());
    }

    let ciphertext = aes_cbc_encrypt(&iv, &key, &input);
    return (iv, ciphertext);
}

fn padding_oracle(iv: &[u8], ciphertext: &[u8]) -> bool {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let plaintext = aes_cbc_decrypt(&iv, &key, &ciphertext);
    return match pkcs7_unpad(&plaintext) {
        Some(_) => true,
        None => false,
    };
}
