/**
 * Crack an MT19937 seed
 * Make sure your MT19937 accepts an integer seed value. Test it (verify that you're getting the
 * same sequence of outputs given a seed).
 *
 * Write a routine that performs the following operation:
 *
 * Wait a random number of seconds between, I don't know, 40 and 1000.
 * Seeds the RNG with the current Unix timestamp
 * Waits a random number of seconds again.
 * Returns the first 32 bit output of the RNG.
 * You get the idea. Go get coffee while it runs. Or just simulate the passage of time, although
 * you're missing some of the fun of this exercise if you do that.
 *
 * From the 32 bit RNG output, discover the seed.
 */

use utils::mt_rand::MtRand;

// use rand;
// use rand::Rng;
// use std::io;
// use std::io::prelude::*;
// use std::thread;
// use std::time::Duration;
use std::thread;
use num_cpus;

use time;
use std::num::Wrapping;

const W: u32 = 32;
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

pub fn run() {
    // // wait random seconds
    // let mut rng = rand::thread_rng();
    // let wait = rng.next_u32() % 60 + 40;
    // println!("Waiting {} seconds", wait);
    // for _ in 0..wait {
    //     thread::sleep(Duration::from_secs(1));
    //     print!(".");
    //     io::stdout().flush().ok();
    // }
    // println!("");

    // pick the current Unix timestamp as seed
    let seed = time::get_time().sec as u32;
    println!("seed: {}", seed);

    let mut mt = MtRand::new_with_seed(seed);
    let first = mt.next();
    assert_eq!(first, mt_rand_first(seed));

    println!("Cracking... (make sure you compile with --release!)");
    let mut threads = Vec::new();
    let num_cpus = num_cpus::get() as u32;
    for id in 0..num_cpus {
        threads.push(thread::spawn(move || {
            println!("thread {} started", id);
            let mut res = Vec::new();
            let mut i: u32 = 0;
            let mut output_at = 0;
            loop {
                // each thread handles 1/num_cpus values.
                if (i % num_cpus) != id {
                    if i == u32::max_value() {
                        break;
                    }
                    i += 1;
                    continue;
                }
                if (id == 0) && (i > output_at) {
                    let done = (10000.0 * (i as f32) / u32::max_value() as f32).round() / 100.0;
                    print!("{}%\r", done);
                    output_at = i + 1000;
                }
                if mt_rand_first(i) == first {
                    println!("\nfound seed: {}", i);
                    res.push(i);
                }
                if i == u32::max_value() {
                    break;
                }
                i += 1;
            }
            println!("thread {} finished", id);
            return res;
        }));
    }
    for t in threads {
        // Wait for the thread to finish. Returns a result.
        let r = t.join();
        println!("result: {:?}", r.unwrap());
    }
}

#[allow(non_snake_case)]
fn mt_rand_first(seed: u32) -> u32 {
    // half init
    let mut a = seed;
    let d = Wrapping(F) * Wrapping(a ^ (a >> (W - 2))) + Wrapping(1);
    a = d.0;
    let state_1 = a;
    for i in 2..M {
        let d = Wrapping(F) * Wrapping(a ^ (a >> (W - 2))) + Wrapping(i as u32);
        a = d.0;
    }
    let d = Wrapping(F) * Wrapping(a ^ (a >> (W - 2))) + Wrapping(M as u32);
    a = d.0;
    let state_M = a;

    // twist only state[0]
    let x = (Wrapping(seed & UPPER_MASK) + Wrapping(state_1 & LOWER_MASK)).0;
    let mut xA = x >> 1;
    if (x % 2) != 0 {
        xA = xA ^ A;
    }
    let y = state_M ^ xA;

    // usual logic
    let y = y ^ ((y >> U) & D);
    let y = y ^ ((y << S) & B);
    let y = y ^ ((y << T) & C);
    let y = y ^ (y >> L);

    return y;
}
