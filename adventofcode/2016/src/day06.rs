// --- Day 6: Signals and Noise ---
//
// Something is jamming your communications with Santa. Fortunately, your signal is only partially
// jammed, and protocol in situations like this is to switch to a simple repetition code to get the
// message through.
//
// In this model, the same message is sent repeatedly. You've recorded the repeating message signal
// (your puzzle input), but the data seems quite corrupted - almost too badly to recover. Almost.
//
// All you need to do is figure out which character is most frequent for each position. For
// example, suppose you had recorded the following messages:
//
// eedadn
// drvtee
// eandsr
// raavrd
// atevrs
// tsrnev
// sdttsa
// rasrtv
// nssdts
// ntnada
// svetve
// tesnvt
// vntsnd
// vrdear
// dvrsen
// enarar
// The most common character in the first column is e; in the second, a; in the third, s, and so
// on. Combining these characters returns the error-corrected message, easter.
//
// Given the recording in your puzzle input, what is the error-corrected version of the message
// being sent?
//
// Your puzzle answer was tzstqsua.
//
// --- Part Two ---
//
// Of course, that would be the message - if you hadn't agreed to use a modified repetition code
// instead.
//
// In this modified code, the sender instead transmits what looks like random data, but for each
// character, the character they actually want to send is slightly less likely than the others.
// Even after signal-jamming noise, you can look at the letter distributions in each column and
// choose the least common letter to reconstruct the original message.
//
// In the above example, the least common character in the first column is a; in the second, d,
// and so on. Repeating this process for the remaining characters produces the original message,
// advent.
//
// Given the recording in your puzzle input and this new decoding methodology, what is the original
// message that Santa is trying to send?
//
// Your puzzle answer was myregdnr.
use std::collections::HashMap;

pub fn solve(input: &str) {
    let test_input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
    assert_eq!(part1(&test_input, 6), "easter");
    println!("part 1: {}", part1(input, 8));

    assert_eq!(part2(&test_input, 6), "advent");
    println!("part 2: {}", part2(input, 8));

}

fn part1(input: &str, l: usize) -> String {
    _solve(&input.trim().split("\n").map(|x| x.as_bytes()).collect(), l).0
}

fn part2(input: &str, l: usize) -> String {
    _solve(&input.trim().split("\n").map(|x| x.as_bytes()).collect(), l).1
}

fn _solve(inputs: &Vec<&[u8]>, l: usize) -> (String, String) {
    // initialize an array of maps. The maps will track the frequency of each character.
    let mut counts = vec![];
    for i in 0..l {
        counts.push(HashMap::new());
    }
    // fill the frequency counts.
    for input in inputs {
        for i in 0..l {
            *counts[i].entry(input[i]).or_insert(0) += 1;
        }
    }
    // find the most and least common character in each map.
    let mut part1 = String::new();
    let mut part2 = String::new();
    for i in 0..l {
        let elements: Vec<(&u8, &i32)> = counts[i].iter().collect();
        let mut min = elements[0];
        let mut max = elements[0];
        for e in elements {
            if e.1 > max.1 {
                max = e;
            }
            if e.1 < min.1 {
                min = e;
            }
        }
        part1.push(*max.0 as char);
        part2.push(*min.0 as char);
    }
    (part1, part2)
}
