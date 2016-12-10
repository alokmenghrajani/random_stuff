// --- Day 4: Security Through Obscurity ---
//
// Finally, you come across an information kiosk with a list of rooms. Of course, the list is
// encrypted and full of decoy data, but the instructions to decode the list are barely hidden
// nearby. Better remove the decoy data first.
//
// Each room consists of an encrypted name (lowercase letters separated by dashes) followed by a
// dash, a sector ID, and a checksum in square brackets.
//
// A room is real (not a decoy) if the checksum is the five most common letters in the encrypted
// name, in order, with ties broken by alphabetization. For example:
//
// aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and
// then a tie between x, y, and z, which are listed alphabetically.
// a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each),
// the first five are listed alphabetically.
// not-a-real-room-404[oarel] is a real room.
// totally-real-room-200[decoy] is not.
// Of the real rooms from the list above, the sum of their sector IDs is 1514.
//
// What is the sum of the sector IDs of the real rooms?
//
// Your puzzle answer was 137896.
//
// --- Part Two ---
//
// With all the decoy data out of the way, it's time to decrypt this list and get moving.
//
// The room names are encrypted by a state-of-the-art shift cipher, which is nearly unbreakable
// without the right software. However, the information kiosk designers at Easter Bunny HQ were not
// expecting to deal with a master cryptographer like yourself.
//
// To decrypt a room name, rotate each letter forward through the alphabet a number of times equal
// to the room's sector ID. A becomes B, B becomes C, Z becomes A, and so on. Dashes become spaces.
//
// For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.
//
// What is the sector ID of the room where North Pole objects are stored?
//
// Your puzzle answer was 501.
extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    assert_eq!(checksum("aaaaa-bbb-z-y-x"), "abxyz");
    assert_eq!(checksum("a-b-c-d-e-f-g-h"), "abcde");
    assert_eq!(checksum("not-a-real-room"), "oarel");
    assert_ne!(checksum("totally-real-room"), "decoy");
    println!("part 1: {}", part1(input));

    assert_eq!(rotate("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
    println!("part 2: {:?}", part2(input));
}

fn part1(input: &str) -> usize {
    _solve(input).0
}

fn part2(input: &str) -> Option<usize> {
    _solve(input).1
}

fn _solve(input: &str) -> (usize, Option<usize>) {
    let mut total = 0;
    let mut north_pole = None;
    for line in input.trim().split("\n") {
        let re = Regex::new(r"([a-z-]*)-(\d+)\[([a-z]{5})\]").unwrap();
        let cap = re.captures(line).unwrap();
        let room = cap.at(1).unwrap();
        let id = cap.at(2).unwrap().parse().unwrap();
        let chksum = cap.at(3).unwrap();
        if checksum(room) == chksum {
            total += id;
        }
        if rotate(room, id) == "northpole object storage" {
            north_pole = Some(id);
        }
    }
    (total, north_pole)
}

fn from_char(c: char) -> usize {
    return ((c as u8) - b'a') as usize;
}

fn to_char(i: usize) -> char {
    return (((i % 26) as u8) + b'a') as char;
}

fn rotate(s: &str, offset: usize) -> String {
    return s.chars()
        .map(|c| if c == '-' {
            ' '
        } else {
            to_char(from_char(c) + offset)
        })
        .collect();
}

fn checksum(s: &str) -> String {
    let mut sums = HashMap::new();
    for c in s.chars() {
        if c == '-' {
            continue;
        }
        *sums.entry(from_char(c)).or_insert(0) += 1;
    }
    // Convert the map<K, V> into a vector of (K, V). We can then sort the vector and return the
    // first 5 elements.
    let mut t: Vec<(&usize, &i32)> = sums.iter().collect();
    t.sort_by(|v1, v2| if v1.1 == v2.1 {
        v1.0.cmp(v2.0)
    } else {
        v2.1.cmp(v1.1)
    });
    t.iter().take(5).fold(String::new(), |acc, v| format!("{}{}", acc, to_char(*v.0)))
}
