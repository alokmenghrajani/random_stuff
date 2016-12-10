// --- Day 5: How About a Nice Game of Chess? ---
//
// You are faced with a security door designed by Easter Bunny engineers that seem to have acquired
// most of their security knowledge by watching hacking movies.
//
// The eight-character password for the door is generated one character at a time by finding the
// MD5 hash of some Door ID (your puzzle input) and an increasing integer index (starting with 0).
//
// A hash indicates the next character in the password if its hexadecimal representation starts
// with five zeroes. If it does, the sixth character in the hash is the next character of the
// password.
//
// For example, if the Door ID is abc:
//
// The first index which produces a hash that starts with five zeroes is 3231929, which we find by
// hashing abc3231929; the sixth character of the hash, and thus the first character of the
// password, is 1.
// 5017308 produces the next interesting hash, which starts with 000008f82..., so the second
// character of the password is 8.
// The third time a hash starts with five zeroes is for abc5278568, discovering the character f.
// In this example, after continuing this search a total of eight times, the password is 18f47a30.
//
// Given the actual Door ID, what is the password?
//
// Your puzzle answer was 4543c154.
//
// --- Part Two ---
//
// As the door slides open, you are presented with a second door that uses a slightly more inspired
// security mechanism. Clearly unimpressed by the last version (in what movie is the password
// decrypted in order?!), the Easter Bunny engineers have worked out a better solution.
//
// Instead of simply filling in the password from left to right, the hash now also indicates the
// position within the password to fill. You still look for hashes that begin with five zeroes;
// however, now, the sixth character represents the position (0-7), and the seventh character is
// the character to put in that position.
//
// A hash result of 000001f means that f is the second character in the password. Use only the
// first result for each position, and ignore invalid positions.
//
// For example, if the Door ID is abc:
//
// The first interesting hash is from abc3231929, which produces 0000015...; so, 5 goes in
// position 1: _5______.
// In the previous method, 5017308 produced an interesting hash; however, it is ignored, because it
// specifies an invalid position (8).
// The second interesting hash is at index 5357525, which produces 000004e...; so, e goes in
// position 4: _5__e___.
// You almost choke on your popcorn as the final character falls into place, producing the password
// 05ace8e3.
//
// Given the actual Door ID and this new method, what is the password? Be extra proud of your
// solution if it uses a cinematic "decrypting" animation.
//
// Your puzzle answer was 1050cbbd.

extern crate crypto;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn solve(input: &str) {
    assert_eq!(part1("abc"), "18f47a30");
    println!("part 1: {}", part1(input));

    assert_eq!(part2("abc"), "05ace8e3");
    println!("part 2: {}", part2(input));
}

fn part1(door: &str) -> String {
    // compute MD5s until we get 8 hashes with 5 leading zeros.
    // we store each character we find in a String.
    let mut res = String::new();
    let mut i = 0;
    while res.len() < 8 {
        match compute_md5(door, i) {
            Some(s) => {
                res.push(s.chars().nth(5).unwrap());
            }
            None => (),
        }
        i += 1;
    }
    res
}

fn part2(door: &str) -> String {
    // compute MD5s until we fill our array.
    let mut res = [b'-'; 8].to_vec();
    let mut found = 0;
    let mut i = 0;
    while found < 8 {
        match compute_md5(door, i) {
            Some(s) => {
                let x = unhex(s.bytes().nth(5).unwrap());
                if x < 8 && res[x as usize] == b'-' {
                    res[x as usize] = s.bytes().nth(6).unwrap();
                    found += 1;
                }
            }
            None => (),
        }
        i += 1;
    }
    String::from_utf8(res).unwrap()
}

// I shouldn't need this...
fn unhex(c: u8) -> u8 {
    if (c >= b'0') && (c <= b'9') {
        return c - b'0';
    }
    return c - b'a' + 10;
}

// md5 computation is very slow if you don't compile the code with --release so use
// `cargo run 5 --release` for this level.
// It would be interesting to wrap the md5 computation in a future and see how much parallelism
// we get out of it.
fn compute_md5(door: &str, i: i32) -> Option<String> {
    let mut md5 = Md5::new();
    md5.reset();
    md5.input_str(door);
    md5.input_str(&i.to_string());
    let s = md5.result_str();
    if s.starts_with("00000") {
        return Some(s);
    }
    return None;
}
