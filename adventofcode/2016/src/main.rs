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
 * Note: the second part of the puzzle is only revealed after you complete part 1. In most cases,
 *       I cleaned up part1. If you are solving these kinds of puzzle with the aim to be the
 *       fastest, you probably wouldn't bother generalizing your code.
 *
 * Credits:
 * - Peter Ruibal whose main.rs I stole.
 * - Matt McPherrin who reviewed some of this code and provided some useful feedback.
 */
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
// todo: clean up and post previous days...
mod day12;
mod day14;
mod day15;
mod day16;
// todo: day17
mod day18;
mod day19;
mod day20;
mod day21;

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

    match day {
        1 => day01::solve(&grab_local_input(day)),
        2 => day02::solve(&grab_local_input(day)),
        3 => day03::solve(&grab_local_input(day)),
        4 => day04::solve(&grab_local_input(day)),
        5 => day05::solve("ojvtpuvg"),
        6 => day06::solve(&grab_local_input(day)),
        7 => day07::solve(&grab_local_input(day)),
        8 => day08::solve(&grab_local_input(day)),
        9 => day09::solve(&grab_local_input(day)),
        10 => day10::solve(&grab_local_input(day)),
        12 => day12::solve(&grab_local_input(day)),
        14 => day14::solve("qzyelonm"),
        15 => day15::solve(&grab_local_input(day)),
        16 => day16::solve("01111001100111011"),
        18 => day18::solve(&grab_local_input(day)),
        19 => day19::solve(3005290),
        20 => day20::solve(&grab_local_input(day)),
        21 => day21::solve(&grab_local_input(day), "abcdefgh", "fbgdceah"),
        _ => panic!("Day {} not implemented", day),
    };
}
