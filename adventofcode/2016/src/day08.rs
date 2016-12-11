// --- Day 8: Two-Factor Authentication ---
//
// You come across a door implementing what you can only assume is an implementation of two-factor
// authentication after a long game of requirements telephone.
//
// To get past the door, you first swipe a keycard (no problem; there was one on a nearby desk).
// Then, it displays a code on a little screen, and you type that code on a keypad. Then,
// presumably, the door unlocks.
//
// Unfortunately, the screen has been smashed. After a few minutes, you've taken everything apart
// and figured out how it works. Now you just have to work out what the screen would have
// displayed.
//
// The magnetic strip on the card you swiped encodes a series of instructions for the screen; these
// instructions are your puzzle input. The screen is 50 pixels wide and 6 pixels tall, all of which
// start off, and is capable of three somewhat peculiar operations:
//
// rect AxB turns on all of the pixels in a rectangle at the top-left of the screen which is A wide
// and B tall.
// rotate row y=A by B shifts all of the pixels in row A (0 is the top row) right by B pixels.
// Pixels that would fall off the right end appear at the left end of the row.
// rotate column x=A by B shifts all of the pixels in column A (0 is the left column) down by B
// pixels. Pixels that would fall off the bottom appear at the top of the column.
// For example, here is a simple sequence on a smaller screen:
//
// rect 3x2 creates a small rectangle in the top-left corner:
//
// ###....
// ###....
// .......
// rotate column x=1 by 1 rotates the second column down by one pixel:
//
// #.#....
// ###....
// .#.....
// rotate row y=0 by 4 rotates the top row right by four pixels:
//
// ....#.#
// ###....
// .#.....
// rotate column x=1 by 1 again rotates the second column down by one pixel, causing the bottom
// pixel to wrap back to the top:
//
// .#..#.#
// #.#....
// .#.....
// As you can see, this display technology is extremely powerful, and will soon dominate the
// tiny-code-displaying-screen market. That's what the advertisement on the back of the display
// tries to convince you, anyway.
//
// There seems to be an intermediate check of the voltage used by the display: after you swipe your
// card, if the screen did work, how many pixels should be lit?
//
// Your puzzle answer was 106.
//
// --- Part Two ---
//
// You notice that the screen is only capable of displaying capital letters; in the font it uses,
// each letter is 5 pixels wide and 6 tall.
//
// After you swipe your card, what code is the screen trying to display?
//
// Your puzzle answer was CFLELOYFCS.
extern crate regex;
use self::regex::Regex;

pub fn solve(input: &str) {
    let mut test_case = Rect::new(7, 3);
    test_case.rect(3, 2);
    assert_eq!(test_case.to_string(), "###....\n###....\n.......\n");

    test_case.rotate_column(1, 1);
    assert_eq!(test_case.to_string(), "#.#....\n###....\n.#.....\n");

    test_case.rotate_row(0, 4);
    assert_eq!(test_case.to_string(), "....#.#\n###....\n.#.....\n");

    test_case.rotate_column(1, 1);
    assert_eq!(test_case.to_string(), ".#..#.#\n#.#....\n.#.....\n");

    println!("part 1: {}", part1(input));
    println!("part 2:\n{}", part2(input));
}

fn part1(input: &str) -> usize {
    _solve(input).chars().map(|x| if x == '#' { 1 } else { 0 }).sum()
}

fn part2(input: &str) -> String {
    _solve(input)
}

fn _solve(input: &str) -> String {
    let mut screen = Rect::new(50, 6);
    let rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let rotate_column = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    for line in input.trim().split("\n") {
        if let Some(cap) = rect.captures(line) {
            screen.rect(cap.at(1).unwrap().parse().unwrap(),
                        cap.at(2).unwrap().parse().unwrap());
        } else if let Some(cap) = rotate_row.captures(line) {
            screen.rotate_row(cap.at(1).unwrap().parse().unwrap(),
                              cap.at(2).unwrap().parse().unwrap());
        } else if let Some(cap) = rotate_column.captures(line) {
            screen.rotate_column(cap.at(1).unwrap().parse().unwrap(),
                                 cap.at(2).unwrap().parse().unwrap());
        } else {
            panic!("unexpected input");
        }
    }
    screen.to_string_nice(true)
}

struct Rect {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl Rect {
    fn new(width: usize, height: usize) -> Rect {
        let mut p = Vec::with_capacity(width);
        for i in 0..width {
            p.push(Vec::with_capacity(height));
            for _ in 0..height {
                p[i].push(false);
            }
        }
        Rect {
            width: width,
            height: height,
            pixels: p,
        }
    }

    fn rect(&mut self, w: usize, h: usize) {
        for i in 0..w {
            for j in 0..h {
                self.pixels[i][j] = true;
            }
        }
    }

    fn transform<F>(&mut self, f: F)
        where F: Fn(usize, usize) -> (isize, isize)
    {
        let mut p = Vec::with_capacity(self.width);
        for i in 0..self.width {
            p.push(Vec::with_capacity(self.height));
            for j in 0..self.height {
                let (mut new_i, mut new_j) = f(i, j);
                new_i = (new_i + self.width as isize) % self.width as isize;
                new_j = (new_j + self.height as isize) % self.height as isize;
                p[i].push(self.pixels[new_i as usize][new_j as usize]);
            }
        }
        self.pixels = p;
    }

    fn rotate_column(&mut self, x: usize, amount: isize) {
        self.transform(|i, j| if i == x {
            (i as isize, j as isize - amount)
        } else {
            (i as isize, j as isize)
        });
    }

    fn rotate_row(&mut self, y: usize, amount: isize) {
        self.transform(|i, j| if j == y {
            (i as isize - amount, j as isize)
        } else {
            (i as isize, j as isize)
        });
    }

    fn to_string(&self) -> String {
        self.to_string_nice(false)
    }

    fn to_string_nice(&self, nice: bool) -> String {
        let mut r = String::new();
        for j in 0..self.height {
            for i in 0..self.width {
                if nice && i % 5 == 0 {
                    r.push_str("  ")
                }
                if self.pixels[i][j] {
                    r.push_str("#")
                } else {
                    r.push_str(if nice { " " } else { "." })
                }
            }
            r.push('\n')
        }
        r
    }
}
