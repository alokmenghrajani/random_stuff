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
use utils::aes::aes_cbc_encrypt;
use utils::aes::aes_cbc_decrypt;

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
