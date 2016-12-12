// --- Day 9: Explosives in Cyberspace ---
//
// Wandering around a secure area, you come across a datalink port to a new part of the network.
// After briefly scanning it for interesting files, you find one file in particular that catches
// your attention. It's compressed with an experimental format, but fortunately, the documentation
// for the format is nearby.
//
// The format compresses a sequence of characters. Whitespace is ignored. To indicate that some
// sequence should be repeated, a marker is added to the file, like (10x2). To decompress this
// marker, take the subsequent 10 characters and repeat them 2 times. Then, continue reading the
// file after the repeated data. The marker itself is not included in the decompressed output.
//
// If parentheses or other characters appear within the data referenced by a marker, that's okay -
// treat it like normal data, not a marker, and then resume looking for markers after the
// decompressed section.
//
// For example:
//
// ADVENT contains no markers and decompresses to itself with no changes, resulting in a
// decompressed length of 6.
// A(1x5)BC repeats only the B a total of 5 times, becoming ABBBBBC for a decompressed length of 7.
// (3x3)XYZ becomes XYZXYZXYZ for a decompressed length of 9.
// A(2x2)BCD(2x2)EFG doubles the BC and EF, becoming ABCBCDEFEFG for a decompressed length of 11.
// (6x1)(1x3)A simply becomes (1x3)A - the (1x3) looks like a marker, but because it's within a
// data section of another marker, it is not treated any differently from the A that comes after
// it. It has a decompressed length of 6.
// X(8x2)(3x3)ABCY becomes X(3x3)ABC(3x3)ABCY (for a decompressed length of 18), because the
// decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed further.
// What is the decompressed length of the file (your puzzle input)? Don't count whitespace.
//
// Your puzzle answer was 110346.
//
// --- Part Two ---
//
// Apparently, the file actually uses version two of the format.
//
// In version two, the only difference is that markers within decompressed data are decompressed.
// This, the documentation explains, provides much more substantial compression capabilities,
// allowing many-gigabyte files to be stored in only a few kilobytes.
//
// For example:
//
// (3x3)XYZ still becomes XYZXYZXYZ, as the decompressed section contains no markers.
// X(8x2)(3x3)ABCY becomes XABCABCABCABCABCABCY, because the decompressed data from the (8x2)
// marker is then further decompressed, thus triggering the (3x3) marker twice for a total of six
// ABC sequences.
// (27x12)(20x12)(13x14)(7x10)(1x12)A decompresses into a string of A repeated 241920 times.
// (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN becomes 445 characters long.
// Unfortunately, the computer you brought probably doesn't have enough memory to actually
// decompress the file; you'll have to come up with another way to get its decompressed length.
//
// What is the decompressed length of the file using this improved format?
//
// Your puzzle answer was 10774309173.
extern crate regex;
use self::regex::Regex;

pub fn solve(input: &str) {
    assert_eq!(part1("ADVENT"), 6);
    assert_eq!(part1("A(1x5)BC"), 7);
    assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(part1("(6x1)(1x3)A"), 6);
    assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    println!("part 1: {}", part1(input));

    assert_eq!(part2("(3x3)XYZ"), 9);
    assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
               445);
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    _solve(input, false)
}

fn part2(input: &str) -> usize {
    _solve(input, true)
}

// Recursively compute the length of our input.
fn _solve(input: &str, recursive: bool) -> usize {
    // We are paying the cost of initializing these regular expressions each time the function
    // gets called. Using static is currently a little painful (requires dealing with raw
    // pointers). Eventually, it will all be easier with const fn.
    let normal = Regex::new(r"^[A-Z]+").unwrap();
    let repeat = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
    let whitespace = Regex::new(r"^\s+").unwrap();

    // Recursion ends when we have consumed the entire input.
    if input.len() == 0 {
        return 0;
    }
    // Check if we have a character in the A-Z range.
    if let Some(n) = normal.captures(input) {
        let l = n.at(0).unwrap().len();
        return l + _solve(&input[l..], recursive);
    }
    // Check if we have a repetition pattern.
    if let Some(r) = repeat.captures(input) {
        let skip = r.at(0).unwrap().len();
        let t: usize = r.at(1).unwrap().parse().unwrap();
        let c: usize = r.at(2).unwrap().parse().unwrap();

        if recursive {
            // for part2, we call the recursion on both pieces
            return c * _solve(&input[skip..skip + t], recursive) +
                   _solve(&input[skip + t..], recursive);
        } else {
            // for part1, we only call the recursion on the second piece
            return c * t + _solve(&input[skip + t..], recursive);
        }
    }
    // Ignore whitespaces
    if whitespace.is_match(input) {
        return 0;
    }
    panic!("unexpected input");
}
