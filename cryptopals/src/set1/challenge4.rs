/**
 * Detect single-character XOR
 * One of the 60-character strings in this file has been encrypted by single-character XOR.
 *
 * Find it.
 *
 * (Your code from #3 should help.)
 */
use set1::challenge1::unhex;
use set1::challenge3::single_byte_xor;
use set1::challenge3::ascii_score;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn run() {
    // load lines from 4.txt
    let file = File::open("4.txt").expect("Unable to open 4.txt");
    let lines = BufReader::new(file).lines();

    let mut best = (0, Vec::new());
    for line in lines {
        let input = unhex(line.unwrap().as_str());
        for i in 0..256 {
            let a = single_byte_xor(&input, i as u8);
            let b = ascii_score(&a);
            if b > best.0 {
                best = (b, a);
            }
        }
    }
    println!("{} {:?}", best.0, best.1);
    let b = String::from_utf8(best.1).unwrap();
    println!("{}", b);
}
