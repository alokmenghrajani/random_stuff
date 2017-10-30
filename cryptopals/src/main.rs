/**
 * https://cryptopals.com/
 */

use std::env;

mod set1;

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
        (1, 1) => set1::challenge1::run(),
        (1, 2) => set1::challenge2::run(),
        _ => panic!("Not implemented"),
    };
}
