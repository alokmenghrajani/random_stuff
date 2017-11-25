/**
 * https://cryptopals.com/
 */

use std::env;

mod set1;
mod set2;
mod set3;
mod set4;
mod utils;

extern crate byteorder;
extern crate crypto;
extern crate rand;
extern crate time;
extern crate num_cpus;

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

        // Set 2
        (2, 9) => set2::challenge9::run(),
        (2, 10) => set2::challenge10::run(),
        (2, 11) => set2::challenge11::run(),
        (2, 12) => set2::challenge12::run(),
        (2, 13) => set2::challenge13::run(),
        (2, 14) => set2::challenge14::run(),
        (2, 15) => set2::challenge15::run(),
        (2, 16) => set2::challenge16::run(),

        // Set 3
        (3, 17) => set3::challenge17::run(),
        (3, 18) => set3::challenge18::run(),
        (3, 19) => set3::challenge19::run(),
        (3, 20) => set3::challenge20::run(),
        (3, 21) => set3::challenge21::run(),
        (3, 22) => set3::challenge22::run(),
        (3, 23) => set3::challenge23::run(),
        (3, 24) => set3::challenge24::run(),

        // Set 4
        (4, 25) => set4::challenge25::run(),
        (4, 26) => set4::challenge26::run(),
        (4, 27) => set4::challenge27::run(),
        (4, 28) => set4::challenge28::run(),
        (4, 29) => set4::challenge29::run(),

        _ => panic!("Not implemented"),
    };
}
