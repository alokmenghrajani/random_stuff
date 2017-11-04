/**
 * AES in ECB mode
 * The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode
 * under the key
 *
 * "YELLOW SUBMARINE".
 * (case-sensitive, without the quotes; exactly 16 characters; I like "YELLOW SUBMARINE"
 * because it's exactly 16 bytes long, and now you do too).
 *
 * Decrypt it. You know the key, after all.
 *
 * Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.
 */

use utils::base64::base64_decode;
use utils::aes::aes_raw_decrypt;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn run() {
    // load lines from 7.txt
    let file = File::open("./src/set1/7.txt").expect("Unable to open 7.txt");
    let lines = BufReader::new(file).lines();
    let contents: String = lines.map(|x| x.unwrap()).collect();
    let data = base64_decode(contents.as_str());

    println!("{:?}", data);

    let key = "YELLOW SUBMARINE".as_bytes();
    let mut final_result = Vec::with_capacity(data.len());
    for block in data.chunks(16) {
        final_result.extend(aes_raw_decrypt(&key, block));
    }
    let plaintext: String = final_result.into_iter().map(|x| x as char).collect();
    println!("{}", plaintext);
}
