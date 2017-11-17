/**
 * Break fixed-nonce CTR statistically
 * In this file find a similar set of Base64'd plaintext. Do with them exactly what you did with
 * the first, but solve the problem differently.
 *
 * Instead of making spot guesses at to known plaintext, treat the collection of ciphertexts the
 * same way you would repeating-key XOR.
 *
 * Obviously, CTR encryption appears different from repeated-key XOR, but with a fixed nonce they
 * are effectively the same thing.
 *
 * To exploit this: take your collection of ciphertexts and truncate them to a common length (the
 * length of the smallest ciphertext will work).
 *
 * Solve the resulting concatenation of ciphertexts as if for repeating- key XOR, with a key size
 * of the length of the ciphertext you XOR'd.
 */

use utils::base64::base64_decode;
use utils::aes::aes_ctr;
use utils::hex::hex_decode;

use set1::challenge3::ascii_score;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn run() {
    let file = File::open("./src/set3/20.txt").expect("Unable to open 20.txt");
    let inputs: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let nonce = [0; 8];
    let mut ciphertexts = Vec::with_capacity(inputs.len());
    let mut max_len = 0;
    let mut plaintexts = Vec::new();
    for input in inputs.iter() {
        let plaintext = base64_decode(input);
        plaintexts.push(String::from_utf8(plaintext).unwrap());
        let ciphertext = aes_ctr(&key, &nonce, &base64_decode(input));
        if ciphertext.len() > max_len {
            max_len = ciphertext.len();
        }
        ciphertexts.push(ciphertext);
    }

    // Break each character. We keep the first ciphertext which yields ascii in the
    // response. We'll re-use the scoring function from set1, challenge 3.
    for i in 0..max_len {
        let mut best = (0, Vec::new());
        for j in 0..256 {
            let mut v = Vec::new();
            for k in 0..ciphertexts.len() {
                if i >= ciphertexts[k].len() {
                    v.push(' ' as u8);
                    continue;
                }
                v.push(ciphertexts[k][i] ^ (j as u8));
            }
            let b = ascii_score(&v);
            if b > best.0 {
                best = (b, v);
            }
        }
        for k in 0..ciphertexts.len() {
            if i >= ciphertexts[k].len() {
                continue;
            }
            ciphertexts[k][i] = best.1[k];
        }
    }

    for i in 0..ciphertexts.len() {
        let t: String = ciphertexts[i].iter().map(|c| *c as char).collect();
        if plaintexts[i] == t {
            println!("ok {}", t);
        } else {
            println!("fail {}\n  vs {}", t, plaintexts[i]);
        }
    }
}
