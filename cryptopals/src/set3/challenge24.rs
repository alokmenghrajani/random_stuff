/**
 * Create the MT19937 stream cipher and break it
 * You can create a trivial stream cipher out of any PRNG; use it to generate a sequence of 8 bit
 * outputs and call those outputs a keystream. XOR each byte of plaintext with each successive byte
 * of keystream.
 *
 * Write the function that does this for MT19937 using a 16-bit seed. Verify that you can encrypt
 * and decrypt properly. This code should look similar to your CTR code.
 *
 * Use your function to encrypt a known plaintext (say, 14 consecutive 'A' characters) prefixed by
 * a random number of random characters.
 *
 * From the ciphertext, recover the "key" (the 16 bit seed).
 *
 * Use the same idea to generate a random "password reset token" using MT19937 seeded from the
 * current time.
 *
 * Write a function to check if any given password token is actually the product of an MT19937 PRNG
 * seeded with the current time.
 */

use utils::mt_rand::MtRand;
use utils::hex::hex_encode;

use rand;
use rand::Rng;

pub fn run() {
    // pick a random seed
    let mut rng = rand::thread_rng();
    let seed = rng.next_u32() as u16;

    println!("seed: {}", seed);

    // encrypt some data
    let plaintext = "hello world".as_bytes();
    let ciphertext = mt_rand_ctr(seed, plaintext);
    println!("ciphertext: {}", hex_encode(&ciphertext));
    let plaintext2 = mt_rand_ctr(seed, &ciphertext);
    println!("plaintext: {}", String::from_utf8(plaintext2).unwrap());

    // known plaintext
    // prepend random bytes (5-10)
    let mut plaintext = Vec::new();
    let n = rng.next_u32() % 5 + 5;
    for _ in 0..n {
        plaintext.push(rng.gen());
    }
    for _ in 0..14 {
        plaintext.push('A' as u8);
    }
    let ciphertext = mt_rand_ctr(seed, &plaintext);
    println!("ciphertext: {}", hex_encode(&ciphertext));

    // break the ciphertext by bruteforcing the seed
    for i in 0..0xffff {
        let t = mt_rand_ctr(i as u16, &ciphertext);
        // check if we found the right seed by look at the last 4 bytes
        let mut ok = true;
        for j in 1..4 {
            if t[t.len() - j] != 'A' as u8 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("possible seed: {}", i);
        }
    }
}

pub fn mt_rand_ctr(seed: u16, input: &[u8]) -> Vec<u8> {
    let mut mt = MtRand::new_with_seed(seed as u32);

    let mut r = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        r.push(mt.next() as u8 ^ input[i as usize]);
    }
    return r;
}
