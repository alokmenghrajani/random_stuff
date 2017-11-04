/**
 * https://cryptopals.com/
 */

use std::env;

mod set1;
mod set2;
mod utils;

extern crate crypto;
extern crate rand;

fn main() {
    // You could simply do `let argv = env::args();` here, but then you can't use square brackets
    // to access elements. Using argv.nth(1) gives you back an Option<String> which must then be
    // unwrapped.
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        panic!("Usage: cargo run <set> <challenge>");
    }

    // In general, Rust can infer types for local variables. Here, we need to tell the type
    // inference which type we want to parse the String into because a String can be parsed into
    // many different types.
    let set: u8 = argv[1].parse().expect("Set must be a number");
    let challenge: u8 = argv[2].parse().expect("Challenge must be a number");
    println!("Running set {}, challenge {}", set, challenge);

    match (set, challenge) {
        // Set 1
        (1, 1) => set1::challenge1::run(),
        (1, 2) => set1::challenge2::run(),
        (1, 3) => set1::challenge3::run(),
        (1, 4) => set1::challenge4::run(),
        (1, 5) => set1::challenge5::run(),
        (1, 6) => set1::challenge6::run(),
        (1, 7) => set1::challenge7::run(),
        (1, 8) => set1::challenge8::run(),
        (2, 9) => set2::challenge9::run(),
        (2, 10) => set2::challenge10::run(),
        (2, 11) => set2::challenge11::run(),
        (2, 12) => set2::challenge12::run(),
        _ => panic!("Not implemented"),
    };
}
