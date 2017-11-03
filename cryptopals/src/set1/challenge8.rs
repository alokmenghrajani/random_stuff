/**
 * Detect AES in ECB mode
 * In this file are a bunch of hex-encoded ciphertexts.
 *
 * One of them has been encrypted with ECB.
 *
 * Detect it.
 *
 * Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte
 * plaintext block will always produce the same 16 byte ciphertext.
 */

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

pub fn run() {
    // load lines from 8.txt
    let file = File::open("./src/set1/8.txt").expect("Unable to open 8.txt");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        let mut blocks = HashSet::new();
        let l = line.clone().into_bytes();
        for block in l.chunks(16) {
            let substr: String = String::from_utf8(block.to_vec()).unwrap();
            if blocks.contains(&substr) {
                println!("Contains dups: {}", line);
                break;
            }
            blocks.insert(substr);
        }
    }
    println!("done");
}
