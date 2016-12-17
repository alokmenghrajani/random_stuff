// --- Day 14: One-Time Pad ---
//
// In order to communicate securely with Santa while you're on this mission, you've been using a
// one-time pad that you generate using a pre-agreed algorithm. Unfortunately, you've run out of
// keys in your one-time pad, and so you need to generate some more.
//
// To generate keys, you first get a stream of random data by taking the MD5 of a pre-arranged salt
// (your puzzle input) and an increasing integer index (starting with 0, and represented in
// decimal); the resulting MD5 hash should be represented as a string of lowercase hexadecimal
// digits.
//
// However, not all of these MD5 hashes are keys, and you need 64 new keys for your one-time pad. A
// hash is a key only if:
//
// It contains three of the same character in a row, like 777. Only consider the first such triplet
// in a hash.
// One of the next 1000 hashes in the stream contains that same character five times in a row, like
// 77777.
// Considering future hashes for five-of-a-kind sequences does not cause those hashes to be
// skipped; instead, regardless of whether the current hash is a key, always resume testing for
// keys starting with the very next hash.
//
// For example, if the pre-arranged salt is abc:
//
// The first index which produces a triple is 18, because the MD5 hash of abc18 contains
// ...cc38887a5.... However, index 18 does not count as a key for your one-time pad, because none
// of the next thousand hashes (index 19 through index 1018) contain 88888.
// The next index which produces a triple is 39; the hash of abc39 contains eee. It is also the
// first key: one of the next thousand hashes (the one at index 816) contains eeeee.
// None of the next six triples are keys, but the one after that, at index 92, is: it contains 999
// and index 200 contains 99999.
// Eventually, index 22728 meets all of the criteria to generate the 64th key.
// So, using our example salt of abc, index 22728 produces the 64th key.
//
// Given the actual salt in your puzzle input, what index produces your 64th one-time pad key?
//
// Your puzzle answer was 15168.
//
// --- Part Two ---
//
// Of course, in order to make this process even more secure, you've also implemented key
// stretching.
//
// Key stretching forces attackers to spend more time generating hashes. Unfortunately, it forces
// everyone else to spend more time, too.
//
// To implement key stretching, whenever you generate a hash, before you use it, you first find the
// MD5 hash of that hash, then the MD5 hash of that hash, and so on, a total of 2016 additional
// hashings. Always use lowercase hexadecimal representations of hashes.
//
// For example, to find the stretched hash for index 0 and salt abc:
//
// Find the MD5 hash of abc0: 577571be4de9dcce85a041ba0410f29f.
// Then, find the MD5 hash of that hash: eec80a0c92dc8a0777c619d9bb51e910.
// Then, find the MD5 hash of that hash: 16062ce768787384c81fe17a7a60c7e3.
// ...repeat many times...
// Then, find the MD5 hash of that hash: a107ff634856bb300138cac6568c0f24.
// So, the stretched hash for index 0 in this situation is a107ff.... In the end, you find the
// original hash (one use of MD5), then find the hash-of-the-previous-hash 2016 times, for a total
// of 2017 uses of MD5.
//
// The rest of the process remains the same, but now the keys are entirely different. Again for
// salt abc:
//
// The first triple (222, at index 5) has no matching 22222 in the next thousand hashes.
// The second triple (eee, at index 10) hash a matching eeeee at index 89, and so it is the first
// key.
// Eventually, index 22551 produces the 64th key (triple fff with matching fffff at index 22859.
// Given the actual salt in your puzzle input and using 2016 extra MD5 calls of key stretching,
// what index now produces your 64th one-time pad key?
//
// Your puzzle answer was 20864.

extern crate crypto;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;
use std::collections::HashMap;

pub fn solve(input: &str) {
    assert_eq!(part1("abc", 1), (b'e', 39, 816));
    println!("part 1: {}", part1(input, 64).1);

    assert_eq!(part2("abc", 1), (b'e', 10, 89));
    println!("part 2: {}", part2(input, 64).1);
}

fn part1(input: &str, n: usize) -> (u8, usize, usize) {
    _solve(input, n, 1)
}

fn part2(input: &str, n: usize) -> (u8, usize, usize) {
    _solve(input, n, 2017)
}

fn _solve(input: &str, n: usize, rounds: usize) -> (u8, usize, usize) {
    let mut i = 0;
    let mut found = 0;
    let mut cache = HashMap::new();
    loop {
        let hash = compute_md5(input, i, rounds, &mut cache).bytes().collect();
        if let Some(c) = find_three_consecutive(hash) {
            if let Some(k) = find_five_consecutive(input, i, c, rounds, &mut cache) {
                found += 1;
                if found == n {
                    return (c, i, k);
                }
            }
        }
        i += 1;
    }
}

fn find_three_consecutive(hash: Vec<u8>) -> Option<u8> {
    for i in 0..(hash.len() - 2) {
        if hash[i] == hash[i + 1] && hash[i] == hash[i + 2] {
            return Some(hash[i]);
        }
    }
    None
}

fn find_five_consecutive(salt: &str,
                         offset: usize,
                         c: u8,
                         rounds: usize,
                         cache: &mut HashMap<usize, String>)
                         -> Option<usize> {
    for j in 1..1001 {
        let hash: Vec<u8> = compute_md5(salt, offset + j, rounds, cache).bytes().collect();
        for i in 0..(hash.len() - 4) {
            if hash[i] == c && hash[i] == hash[i + 1] && hash[i] == hash[i + 2] &&
               hash[i] == hash[i + 3] && hash[i] == hash[i + 4] {
                return Some(offset + j);
            }
        }
    }
    None
}

fn compute_md5(salt: &str, i: usize, rounds: usize, cache: &mut HashMap<usize, String>) -> String {
    if let Some(r) = cache.get(&i) {
        return r.clone();
    }
    let mut md5 = Md5::new();
    let mut r = format!("{}{}", salt, i);
    for _ in 0..rounds {
        md5.reset();
        md5.input_str(&r[..]);
        r = md5.result_str();
    }
    cache.insert(i, r.clone());
    r
}
