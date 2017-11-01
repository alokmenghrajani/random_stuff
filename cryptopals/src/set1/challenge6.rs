/**
 * Break repeating-key XOR
 *
 * There's a file here. It's been base64'd after being encrypted with repeating-key XOR.
 *
 * Decrypt it.
 * ...
 *
 * The challenge comes with step-by-step instructions but I decided to give it a try
 * with my own ideas.
 * A xor K xor B xor K == A xor B.
 * All the bytes have the MSB at 0. This implies the text and key are ascii.
 * We can try to xor the data with itself shift by N bytes.
 */

use utils::base64::base64_decode;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn run() {
    // load lines from 6.txt
    let file = File::open("./src/set1/6.txt").expect("Unable to open 6.txt");
    let lines = BufReader::new(file).lines();
    let contents: String = lines.map(|x| x.unwrap()).collect();
    let data = base64_decode(contents.as_str());

    for i in 2..40 {
        let d = xor_with_shift(&data, i);
        println!("{:?}", d);
        print!("{}: ", i);
        for b in d {
            print!("{}", (b as i32) >> 8);
        }
        println!("");
    }
}

fn xor_with_shift(input: &Vec<u8>, shift: i32) -> Vec<u8> {
    let mut r = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let j = (i + shift as usize) % input.len();
        r.push(input[i] ^ input[j]);
    }
    return r;
}
