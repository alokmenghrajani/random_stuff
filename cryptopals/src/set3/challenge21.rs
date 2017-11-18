/**
 * Implement the MT19937 Mersenne Twister RNG
 * You can get the psuedocode for this from Wikipedia.
 *
 * If you're writing in Python, Ruby, or (gah) PHP, your language is probably already giving you
 * MT19937 as "rand()"; don't use rand(). Write the RNG yourself.
 */

use utils::mt_rand::MtRand;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

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
