// --- Day 3: Squares With Three Sides ---
//
// Now that you can think clearly, you move deeper into the labyrinth of hallways and office
// furniture that makes up this part of Easter Bunny HQ. This must be a graphic design department;
// the walls are covered in specifications for triangles.
//
// Or are they?
//
// The design document gives the side lengths of each triangle it describes, but... 5 10 25? Some
// of these aren't triangles. You can't help but mark the impossible ones.
//
// In a valid triangle, the sum of any two sides must be larger than the remaining side. For
// example, the "triangle" given above is impossible, because 5 + 10 is not larger than 25.
//
// In your puzzle input, how many of the listed triangles are possible?
//
// Your puzzle answer was 982.
//
// --- Part Two ---
//
// Now that you've helpfully marked up their design documents, it occurs to you that triangles are
// specified in groups of three vertically. Each set of three numbers in a column specifies a
// triangle. Rows are unrelated.
//
// For example, given the following specification, numbers with the same hundreds digit would be
// part of the same triangle:
//
// 101 301 501
// 102 302 502
// 103 303 503
// 201 401 601
// 202 402 602
// 203 403 603
// In your puzzle input, and instead reading by columns, how many of the listed triangles are
// possible?
//
// Your puzzle answer was 1826.
extern crate regex;
use self::regex::Regex;

pub fn solve(input: &str) {
    assert_eq!(part1("5 10 25\n10 5 25\n5 10 25\n5 10 25\n12 5 10"), 1);
    println!("part 1: {}", part1(input));

    assert_eq!(part2("101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 \
                      603"),
               6);
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    _solve(input, 1)
}

fn part2(input: &str) -> usize {
    _solve(input, 3)
}

// The dir as a usize is a very unclean way to solve this puzzle.
fn _solve(input: &str, dir: usize) -> usize {
    let mut data: Vec<usize> = vec![];
    let re = Regex::new(r"\s*(\d+)").unwrap();
    for line in input.trim().split("\n") {
        // TODO: is there a simpler way to parse the input besides using a regexp? The regexp
        // implies some ugly unwrap() + parse() + expect().
        for cap in re.captures_iter(line) {
            data.push(cap.at(1).unwrap().parse().expect("expecting a number"));
        }
    }
    let mut i = 0;
    let mut total = 0;
    while i < data.len() {
        for j in 0..dir {
            let mut v = vec![data[i + j], data[i + j + dir], data[i + j + 2 * dir]];
            v.sort();
            if v[0] + v[1] > v[2] {
                total += 1;
            }
        }
        i += dir * 3;
    }
    total
}
