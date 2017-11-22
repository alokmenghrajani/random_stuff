/**
 * Break "random access read/write" AES CTR
 * Back to CTR. Encrypt the recovered plaintext from this file (the ECB exercise) under CTR with a
 * random key (for this exercise the key should be unknown to you, but hold on to it).
 *
 * Now, write the code that allows you to "seek" into the ciphertext, decrypt, and re-encrypt with
 * different plaintext. Expose this as a function, like, "edit(ciphertext, key, offset, newtext)".
 *
 * Imagine the "edit" function was exposed to attackers by means of an API call that didn't reveal
 * the key or the original plaintext; the attacker has the ciphertext and controls the offset and
 * "new text".
 *
 * Recover the original plaintext.
 *
 * Food for thought.
 * A folkloric supposed benefit of CTR mode is the ability to easily "seek forward" into the
 * ciphertext; to access byte N of the ciphertext, all you need to be able to do is generate byte N
 * of the keystream. Imagine if you'd relied on that advice to, say, encrypt a disk.
 */

use utils::aes::aes_ctr;
use utils::aes::aes_ecb_decrypt;
use utils::base64::base64_decode;
use utils::hex::hex_decode;
use utils::xor::xor;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    // load lines from 25.txt
    let file = File::open("./src/set4/25.txt").expect("Unable to open 25.txt");
    let lines = BufReader::new(file).lines();
    let contents: String = lines.map(|x| x.unwrap()).collect();
    let data = base64_decode(contents.as_str());

    let key = "YELLOW SUBMARINE".as_bytes();
    let plaintext = aes_ecb_decrypt(&key, &data);
    let plaintext1: String = plaintext.clone().into_iter().map(|x| x as char).collect();

    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let nonce = [0; 8];
    let ciphertext = aes_ctr(&key, &nonce, &plaintext);

    // easiest way to break this is to edit the ciphertext with all 'A'
    let mut a = Vec::new();
    for _ in 0..ciphertext.len() {
        a.push('A' as u8);
    }
    let ciphertext2 = edit(&ciphertext, &key, 0, &a);

    let plaintext = xor(&xor(&ciphertext2, &ciphertext), &a);
    let plaintext2: String = plaintext.into_iter().map(|x| x as char).collect();
    println!("plaintext 2");
    println!("{}", plaintext2);

    assert_eq!(plaintext1, plaintext2);
}

/**
 * We could refactor aes_ctr to allow encrypting specific blocks. It's easier to just decrypt,
 * modify the buffer and re-encrypt.
 */
fn edit(ciphertext: &[u8], key: &[u8], offset: usize, newtext: &[u8]) -> Vec<u8> {
    let nonce = [0; 8];
    let mut plaintext = aes_ctr(&key, &nonce, &ciphertext);
    for i in 0..newtext.len() {
        plaintext[i + offset] = newtext[i];
    }
    return aes_ctr(&key, &nonce, &plaintext);
}
