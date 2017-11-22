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
 *
 * A xor K xor B xor K == A xor B.
 * I xor the data with itself, shifted by N bytes. I then compare the distribution of
 * bytes with a reference text which gets the same transformation.
 */

use utils::base64::base64_decode;
use utils::xor::xor;
use set1::challenge5::repeated_xor;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::f32;

pub fn run() {
    // load lines from 6.txt
    let file = File::open("./src/set1/6.txt").expect("Unable to open 6.txt");
    let lines = BufReader::new(file).lines();
    let contents: String = lines.map(|x| x.unwrap()).collect();
    let data = base64_decode(contents.as_str());

    // load some reference text
    let mut file = File::open("./src/set1/english.txt").expect("Unable to open english.txt");
    let mut english = Vec::new();
    file.read_to_end(&mut english).expect("unable to read file");

    println!("{:?}", data);

    // the best result is when the score is minimal
    println!("Guessing key length");
    let mut best = (f32::MAX, 0);
    for i in 2..41 {
        let d = xor_with_shift(&data[..], i);
        let hist1 = histogram(&d[..]);

        let e = xor_with_shift(&english[..], i);
        let hist2 = histogram(&e[..]);

        let score = histogram_diff(hist1, hist2);
        if score < best.0 {
            best = (score, i);
        }
        println!("{} {}", i, score);
    }
    println!("Best: {:?}", best);
    println!("");

    // now break the key, one character at a time
    let hist = histogram(&english[..]);

    let j = best.1;
    let mut key = String::new();
    for i in 0..j {
        // take every jth character from data
        let mut d = Vec::with_capacity(data.len() / (j as usize));
        let mut k = i as usize;
        while k < data.len() {
            d.push(data[k]);
            k += j as usize;
        }
        let mut best = (f32::MAX, 257);
        for k in 0..256 {
            let e = xor(&d, &[k as u8]);
            let f = histogram(&e[..]);
            let score = histogram_diff(hist, f);
            if score < best.0 {
                best = (score, k);
            }
        }
        key.push(best.1 as u8 as char);
        println!("{} {} {}", i, best.0, best.1);
    }
    println!("key: {}", key);

    let plaintext = repeated_xor(&data, &key.into_bytes());
    let p: String = plaintext.into_iter().map(|x| x as char).collect();
    println!("{}", p);
}

fn histogram_diff(a: [f32; 256], b: [f32; 256]) -> f32 {
    let mut diff = 0.0;
    for (l, r) in a.iter().zip(b.iter()) {
        diff += (l - r) * (l - r);
    }
    return diff;
}

fn histogram(input: &[u8]) -> [f32; 256] {
    let mut histogram = [0.0; 256];

    for i in input {
        histogram[*i as usize] += 1.0;
    }

    let sum: f32 = histogram.iter().sum();
    for x in histogram.iter_mut() {
        *x /= sum;
    }

    return histogram;
}

fn xor_with_shift(input: &[u8], shift: i32) -> Vec<u8> {
    let mut r = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let j = (i + shift as usize) % input.len();
        r.push(input[i] ^ input[j]);
    }
    return r;
}
