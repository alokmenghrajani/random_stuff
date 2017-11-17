/**
 * Implement the MT19937 Mersenne Twister RNG
 * You can get the psuedocode for this from Wikipedia.
 *
 * If you're writing in Python, Ruby, or (gah) PHP, your language is probably already giving you
 * MT19937 as "rand()"; don't use rand(). Write the RNG yourself.
 */

use std::num::Wrapping;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;
const A: u32 = 0x9908B0DF;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const F: u32 = 1812433253;
const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = 1 << 31;

pub struct MtRand {
    state: [u32; N],
    index: usize,
}

impl MtRand {
    pub fn new_with_seed(seed: u32) -> MtRand {
        let mut mt = MtRand {
            state: [0; N],
            index: N,
        };
        mt.state[0] = seed;
        for i in 1..N {
            let a = mt.state[i - 1];
            let d = Wrapping(F) * Wrapping(a ^ (a >> (W - 2))) + Wrapping(i as u32);
            mt.state[i] = d.0;
        }
        return mt;
    }

    pub fn next(&mut self) -> u32 {
        if self.index == N {
            self.twist();
        }

        let mut y = self.state[self.index];
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.index += 1;
        return y;
    }

    #[allow(non_snake_case)]
    fn twist(&mut self) {
        for i in 0..N {
            let x = (Wrapping(self.state[i] & UPPER_MASK) + Wrapping(self.get(i + 1) & LOWER_MASK))
                .0;
            let mut xA = x >> 1;
            if (x % 2) != 0 {
                xA = xA ^ A;
            }
            self.state[i] = self.get(i + M) ^ xA;
        }
        self.index = 0;
    }

    fn get(&self, idx: usize) -> u32 {
        let offset = idx % N;
        return self.state[offset];
    }
}

pub fn run() {
    // I'm comparing the output of my implementation with PHP. But PHP is lolzy in two ways:
    // 1. you need to pick MT_RAND_MT19937 (as opposed to MT_RAND_PHP) when initializing things
    //    (because the Mersenne Twister implementation was previously incorrect and they kept a
    //    flag to pick between the broken and correct implementations).
    // 2. the result is right shifted by one bit.

    let file = File::open("./src/set3/21.test_vector").expect("Unable to open 21.test_vector");
    let inputs: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let seed = 1234;
    println!("seed: {}", seed);
    let mut mt = MtRand::new_with_seed(seed);
    let mut ok = 0;
    for i in 0..inputs.len() {
        let r = mt.next();
        let s = inputs[i].parse::<u32>().unwrap();
        if (r >> 1) == s {
            ok += 1;
        }
        println!("{} {}", i, r);
    }
    println!("Result: {} / {}", ok, inputs.len());
}
