/**
 * Advent of code is an annual programming challenge hosted by Eric Wastl.
 *
 * This year, I decided to solve the puzzles in Rust as a way to learn a new language. As a result,
 * the code should be overly commented and useful for anyone else starting to write Rust code.
 *
 * Step 1: install Rust -> https://www.rust-lang.org/en-US/
 * Step 2: rtfm -> https://doc.rust-lang.org/book/
 * Step 3: solve the puzzles starting from http://adventofcode.com/2016/day/1
 *
 * Credits:
 * - Peter Ruibal whose main.rs I stole.
 * - Matt McPherrin who reviewed some of this code and provided some useful feedback.
 */

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod day01;

// Loads the file ./input/<number>. The input files are unique to each person.
// All the inputs seem to be ascii, it might be more efficient to use [u8]?
fn grab_local_input(number: u8) -> String {
    let path = format!("./input/{}", number);
    let mut fp = File::open(&path).expect(&format!("Can't open {}", path));
    let mut buf = String::new();
    fp.read_to_string(&mut buf).unwrap();
    buf
}

fn main() {
    // You could simply do `let argv = env::args();` here, but then you can't use square brackets
    // to access elements. Using argv.nth(1) gives you back an Option<String> which must then be
    // unwrapped.
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("Usage: cargo run <day>");
    }

    // In general, Rust can infer types for local variables. Here, we need to tell the type
    // inference which type we want to parse the String into because a String can be parsed into
    // many different types.
    let day: u8 = argv[1].parse().expect("Day must be a number");
    println!("Running day {}", day);

    let input = grab_local_input(day);
    match day {
        1 => day01::solve(&input),
        // 2 => day02::solve(&grab_local_input(2)),
        // 3 => day03::solve(&grab_local_input(3)),
        // 4 => day04::solve(&grab_local_input(4)),
        // 5 => day05::solve(&grab_local_input(5)),
        // 6 => day06::solve(&grab_local_input(6)),
        _ => panic!("Day {} not implemented", day),
    };
}
