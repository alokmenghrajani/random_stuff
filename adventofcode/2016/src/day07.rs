// --- Day 7: Internet Protocol Version 7 ---
//
// While snooping around the local network of EBHQ, you compile a list of IP addresses (they're
// IPv7, of course; IPv6 is much too limited). You'd like to figure out which IPs support TLS
// (transport-layer snooping).
//
// An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An ABBA is any
// four-character sequence which consists of a pair of two different characters followed by the
// reverse of that pair, such as xyyx or abba. However, the IP also must not have an ABBA within
// any hypernet sequences, which are contained by square brackets.
//
// For example:
//
// abba[mnop]qrst supports TLS (abba outside square brackets).
// abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even though xyyx is outside
// square brackets).
// aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters must be
// different).
// ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even though it's within a
// larger string).
// How many IPs in your puzzle input support TLS?
//
// Your puzzle answer was 110.
//
// --- Part Two ---
//
// You would also like to know which IPs support SSL (super-secret listening).
//
// An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in the supernet
// sequences (outside any square bracketed sections), and a corresponding Byte Allocation Block, or
// BAB, anywhere in the hypernet sequences. An ABA is any three-character sequence which consists
// of the same character twice with a different character between them, such as xyx or aba. A
// corresponding BAB is the same characters but in reversed positions: yxy and bab, respectively.
//
// For example:
//
// aba[bab]xyz supports SSL (aba outside square brackets with corresponding bab within square
// brackets).
// xyx[xyx]xyx does not support SSL (xyx, but no corresponding yxy).
// aaa[kek]eke supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence
// is not related, because the interior character must be different).
// zazbz[bzb]cdb supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even
// though zaz and zbz overlap).
// How many IPs in your puzzle input support SSL?
//
// Your puzzle answer was 242.
pub fn solve(input: &str) {
    assert!(do_part1("abba[mnop]qrst"));
    assert!(!do_part1("abcd[bddb]xyyx"));
    assert!(!do_part1("aaaa[qwer]tyui"));
    assert!(do_part1("ioxxoj[asdfgh]zxcvbn"));
    println!("part 1: {}", part1(input));

    assert!(do_part2("aba[bab]xyz"));
    assert!(!do_part2("xyx[xyx]xyx"));
    assert!(do_part2("aaa[kek]eke"));
    assert!(do_part2("zazbz[bzb]cdb"));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.trim().split("\n") {
        if do_part1(line) {
            total += 1;
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    for line in input.trim().split("\n") {
        if do_part2(line) {
            total += 1;
        }
    }
    total
}

// split the input, then
// - return false if a hypernet contains ABBA
// - return true if a supernet contains ABBA
fn do_part1(input: &str) -> bool {
    let t = split(input);
    let supernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 0 { Some(&x.0) } else { None }).collect();
    let hypernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 1 { Some(&x.0) } else { None }).collect();
    if hypernets.iter().any(|x| contains_abba(*x)) {
        return false;
    }
    if supernets.iter().any(|x| contains_abba(*x)) {
        return true;
    }
    false
}

// split the input, then
// - find all the ABA in supernets
// - check if BAB exists in hypernets
fn do_part2(input: &str) -> bool {
    let t = split(input);
    let supernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 0 { Some(&x.0) } else { None }).collect();
    let hypernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 1 { Some(&x.0) } else { None }).collect();
    let abas = get_aba(supernets);
    for aba in abas {
        for hypernet in hypernets.iter() {
            if contains(hypernet, aba) {
                return true;
            }
        }
    }
    false
}

// Really ugly code. Didn't manage to find a better approach here...
// contains, get_aba and contains_abba can probably refactored into a single function. Perhaps
// loop over all pairs of characters and check if ABBA, ABA and BAB exists?
fn contains(hypernet: &String, aba: (u8, u8)) -> bool {
    let t = hypernet.as_bytes();
    for i in 0..t.len() - 2 {
        if t[i] == aba.1 && t[i + 1] == aba.0 && t[i + 2] == aba.1 {
            return true;
        }
    }
    false
}

fn get_aba(inputs: Vec<&String>) -> Vec<(u8, u8)> {
    let mut r = Vec::new();
    for input in inputs.iter() {
        let t = input.as_bytes();
        for i in 0..t.len() - 2 {
            if t[i] == t[i + 2] && t[i] != t[i + 1] {
                r.push((t[i], t[i + 1]));
            }
        }
    }
    r
}

fn contains_abba(input: &String) -> bool {
    let t = input.as_bytes();
    for i in 0..t.len() - 3 {
        if t[i] == t[i + 3] && t[i + 1] == t[i + 2] && t[i] != t[i + 1] {
            return true;
        }
    }
    false
}

// Given a string of the form: foo[bar]abc[xyz]blah this code returns a vector which gives the
// depth for each piece of string. I.e. this code can handle crazy strings of the form
// foo[bar[abc]xyz]blah, which don't actually occure in the puzzle.
fn split(input: &str) -> Vec<(String, usize)> {
    let mut r = vec![];
    let mut buf = String::new();
    let mut depth = 0;
    for c in input.chars() {
        if c == '[' {
            if buf != "" {
                r.push((buf, depth));
                buf = String::new();
            }
            depth += 1;
        } else if c == ']' {
            if buf != "" {
                r.push((buf, depth));
                buf = String::new();
            }
            depth -= 1;
        } else {
            buf.push(c);
        }
    }
    if buf != "" {
        r.push((buf, depth));
    }
    r
}
