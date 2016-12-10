// --- Day 1: No Time for a Taxicab ---
//
// Santa's sleigh uses a very high-precision clock to guide its movements, and the clock's
// oscillator is regulated by stars. Unfortunately, the stars have been stolen... by the Easter
// Bunny. To save Christmas, Santa needs you to retrieve all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the advent
// calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one
// star. Good luck!
//
// You're airdropped near Easter Bunny Headquarters in a city somewhere. "Near", unfortunately, is
// as close as you can get - the instructions on the Easter Bunny Recruiting Document the Elves
// intercepted start here, and nobody had time to work them out further.
//
// The Document indicates that you should start at the given coordinates (where you just landed)
// and face North. Then, follow the provided sequence: either turn left (L) or right (R) 90
// degrees, then walk forward the given number of blocks, ending at a new intersection.
//
// There's no time to follow such ridiculous instructions on foot, though, so you take a moment and
// work out the destination. Given that you can only walk on the street grid of the city, how far
// is the shortest path to the destination?
//
// For example:
//
// Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
// R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
// R5, L5, R5, R3 leaves you 12 blocks away.
// How many blocks away is Easter Bunny HQ?
//
// Your puzzle answer was 243.
//
// --- Part Two ---
//
// Then, you notice the instructions continue on the back of the Recruiting Document. Easter Bunny
// HQ is actually at the first location you visit twice.
//
// For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4
// blocks away, due East.
//
// How many blocks away is the first location you visit twice?
//
// Your puzzle answer was 142.
use std;

pub fn solve(input: &str) {
    assert_eq!(part1("R2, L3"), 5);
    assert_eq!(part1("R2, R2, R2"), 2);
    assert_eq!(part1("R5, L5, R5, R3"), 12);
    println!("part 1: {}", part1(input));

    assert_eq!(part2("R8, R4, R4, R8"), 4);
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> isize {
    _solve(input).0
}

fn part2(input: &str) -> isize {
    _solve(input).1
}

fn _solve(input: &str) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dir: isize = 0;
    let mut visited: Vec<(isize, isize)> = Vec::new();
    let mut first_visited = None;

    for substr in input.trim().split(", ") {
        match substr.chars().next().unwrap() {
            'R' => dir += 1,
            'L' => dir -= 1,
            _ => panic!("expecting R or L"),
        };
        let value: i32 = substr[1..].parse().expect("number must follow R or L");
        for _ in 0..value {
            x += ((dir as f64) * std::f64::consts::PI / 2.0).sin().round() as isize;
            y += ((dir as f64) * std::f64::consts::PI / 2.0).cos().round() as isize;
            if first_visited == None {
                for &(tx, ty) in &visited {
                    if tx == x && ty == y {
                        first_visited = Some(dist(x, y));
                        break;
                    }
                }
            }
            visited.push((x, y));
        }
    }
    match first_visited {
        Some(v) => (dist(x, y), v),
        None => (dist(x, y), -1),
    }
}

fn dist(x: isize, y: isize) -> isize {
    x.abs() + y.abs()
}
