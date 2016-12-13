// --- Day 10: Balance Bots ---
//
// You come upon a factory in which many robots are zooming around handing small microchips to each
// other.
//
// Upon closer examination, you notice that each bot only proceeds when it has two microchips, and
// once it does, it gives each one to a different bot or puts it in a marked "output" bin.
// Sometimes, bots take microchips from "input" bins, too.
//
// Inspecting one of the microchips, it seems like they each contain a single number; the bots must
// use some logic to decide what to do with each chip. You access the local control computer and
// download the bots' instructions (your puzzle input).
//
// Some of the instructions specify that a specific-valued microchip should be given to a specific
// bot; the rest of the instructions indicate what a given bot should do with its lower-value or
// higher-value chip.
//
// For example, consider the following instructions:
//
// value 5 goes to bot 2
// bot 2 gives low to bot 1 and high to bot 0
// value 3 goes to bot 1
// bot 1 gives low to output 1 and high to bot 0
// bot 0 gives low to output 2 and high to output 0
// value 2 goes to bot 2
// Initially, bot 1 starts with a value-3 chip, and bot 2 starts with a value-2 chip and a value-5
// chip.
// Because bot 2 has two microchips, it gives its lower one (2) to bot 1 and its higher one (5) to
// bot 0.
// Then, bot 1 has two microchips; it puts the value-2 chip in output 1 and gives the value-3 chip
// to bot 0.
// Finally, bot 0 has two microchips; it puts the 3 in output 2 and the 5 in output 0.
// In the end, output bin 0 contains a value-5 microchip, output bin 1 contains a value-2
// microchip, and output bin 2 contains a value-3 microchip. In this configuration, bot number 2 is
// responsible for comparing value-5 microchips with value-2 microchips.
//
// Based on your instructions, what is the number of the bot that is responsible for comparing
// value-61 microchips with value-17 microchips?
//
// Your puzzle answer was 47.
//
// --- Part Two ---
//
// What do you get if you multiply together the values of one chip in each of outputs 0, 1, and 2?
//
// Your puzzle answer was 2666.

extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let test_input = "value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 \
                      goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives \
                      low to output 2 and high to output 0\nvalue 2 goes to bot 2";
    assert_eq!(part1(test_input, 2, 5), 2);
    println!("part 1: {}", part1(input, 17, 61));

    assert_eq!(part2(test_input, 0), 5);
    assert_eq!(part2(test_input, 1), 2);
    println!("part 2: {}",
             part2(input, 0) * part2(input, 1) * part2(input, 2));
}

fn part1(input: &str, value1: usize, value2: usize) -> usize {
    _solve(input, value1, value2, 0).0
}

fn part2(input: &str, value: usize) -> usize {
    _solve(input, 0, 0, value).1
}

// For now, I'm not using any fun way to solve this puzzle. At some point, I should re-visit and
// implement each bot as a thread.

// We could do without Dest and have output be a special bot which can hold more than 2 items.
#[derive(Clone, Copy)]
enum Dest {
    Bot(usize),
    Output(usize),
}

// We only need to think about two commands.
enum Command {
    Take(usize, usize),
    Give(usize, Dest, Dest),
}

// A bot contains chips. I used left-right, but I should have probably just made this a Vec<usize>.
struct Bot {
    left: Option<usize>,
    right: Option<usize>,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            left: None,
            right: None,
        }
    }

    fn has_two_chips(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    fn get(&mut self) -> (usize, usize) {
        let l;
        let h;
        if self.left.unwrap() > self.right.unwrap() {
            h = self.left.unwrap();
            l = self.right.unwrap();
        } else {
            l = self.left.unwrap();
            h = self.right.unwrap();
        }
        self.left = None;
        self.right = None;
        (l, h)
    }

    fn put(&mut self, value: usize) {
        match (self.left, self.right) {
            (None, _) => self.left = Some(value),
            (_, None) => self.right = Some(value),
            _ => panic!("bot already has two chips"),
        }
    }
}

fn _solve(input: &str, value1: usize, value2: usize, bin: usize) -> (usize, usize) {
    // Parse the commands
    let mut commands: Vec<Command> = vec![];
    let take = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let give =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
            .unwrap();
    for line in input.trim().split('\n') {
        if let Some(cap) = take.captures(line) {
            commands.push(Command::Take(cap.at(1).unwrap().parse().unwrap(),
                                        cap.at(2).unwrap().parse().unwrap()));
            continue;
        }
        if let Some(cap) = give.captures(line) {
            let src_bot = cap.at(1).unwrap().parse().unwrap();
            let low_n = cap.at(3).unwrap().parse().unwrap();
            let low;
            if cap.at(2).unwrap() == "bot" {
                low = Dest::Bot(low_n);
            } else {
                low = Dest::Output(low_n);
            }
            let high_n = cap.at(5).unwrap().parse().unwrap();
            let high;
            if cap.at(4).unwrap() == "bot" {
                high = Dest::Bot(high_n);
            } else {
                high = Dest::Output(high_n);
            }
            commands.push(Command::Give(src_bot, low, high));
            continue;
        }
        panic!("unexpected input");
    }

    let mut bots = HashMap::new();
    let mut part1 = 0;
    let mut output = HashMap::new();
    loop {
        let mut new_commands = vec![];
        if commands.len() == 0 {
            break;
        }
        for command in commands {
            match command {
                Command::Take(v, b) => {
                    bots.entry(b).or_insert(Bot::new()).put(v);
                    // println!("bot {} is taking {}", b, v);
                }
                Command::Give(src, low, high) => {
                    let (l, h) = {
                        let bot = bots.entry(src).or_insert(Bot::new());
                        if !bot.has_two_chips() {
                            new_commands.push(command);
                            continue;
                        }
                        bot.get()
                    };
                    // println!("bot {} is giving {} and {}", src, l, h);
                    if l == value1 && h == value2 {
                        part1 = src;
                    }
                    if let Dest::Bot(low) = low {
                        bots.entry(low).or_insert(Bot::new()).put(l);
                    } else if let Dest::Output(low) = low {
                        output.insert(low, l);
                    } else {
                        panic!("unreachable");
                    }
                    if let Dest::Bot(high) = high {
                        bots.entry(high).or_insert(Bot::new()).put(h);
                    } else if let Dest::Output(high) = high {
                        output.insert(high, h);
                    } else {
                        panic!("unreachable");
                    }
                }
            }
        }
        commands = new_commands;
    }
    (part1, *output.get(&bin).unwrap())
}
