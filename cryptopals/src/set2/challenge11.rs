/**
 * An ECB/CBC detection oracle
 * Now that you have ECB and CBC working:
 *
 * Write a function to generate a random AES key; that's just 16 random bytes.
 *
 * Write a function that encrypts data under an unknown key --- that is, a function that generates
 * a random key and encrypts under it.
 *
 * The function should look like:
 *
 * encryption_oracle(your-input)
 * => [MEANINGLESS JIBBER JABBER]
 * Under the hood, have the function append 5-10 bytes (count chosen randomly) before the plaintext
 * and 5-10 bytes after the plaintext.
 *
 * Now, have the function choose to encrypt under ECB 1/2 the time, and under CBC the other half
 * (just use random IVs each time for CBC). Use rand(2) to decide which to use.
 *
 * Detect the block cipher mode the function is using each time. You should end up with a piece of
 * code that, pointed at a block box that might be encrypting ECB or CBC, tells you which one is
 * happening.
 */

use utils::aes::aes_cbc_encrypt;
use utils::aes::aes_ecb_encrypt;
use utils::hex::hex_encode;

use rand::Rng;
use rand;
use std::collections::HashSet;

pub fn run() {
    let n = 100;
    let mut ok = 0;
    let repeated_text = "yellow submarineyellow submarineyellow submarine";

    for _ in 0..n {
        let (m, ciphertext) = encrypt_with_random_key(repeated_text.as_bytes());
        let g = guess_mode(&ciphertext);
        println!("{:?} {:?}", m, g);
        if m == g {
            ok += 1;
        }
    }
    println!("result: {} / {}", ok, n);
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    ECB,
    CBC,
}

pub fn guess_mode(input: &[u8]) -> Mode {
    let mut blocks = HashSet::new();
    let l = hex_encode(input);
    for block in l.as_bytes().chunks(32) {
        let substr: String = String::from_utf8(block.to_vec()).unwrap();
        if blocks.contains(&substr) {
            return Mode::ECB;
        }
        blocks.insert(substr);
    }
    return Mode::CBC;
}

fn encrypt_with_random_key(input: &[u8]) -> (Mode, Vec<u8>) {
    let mut rng = rand::thread_rng();

    // generate a random key
    let mut key = [0; 16];
    rng.fill_bytes(&mut key);

    // prepend random bytes (5-10)
    let mut plaintext = Vec::new();
    let n = rng.next_u32() % 5 + 5;
    for _ in 0..n {
        plaintext.push(rng.gen());
    }

    // copy choosen input
    plaintext.append(&mut input.to_vec());

    // append random bytes (5-10)
    let n = rng.next_u32() % 5 + 5;
    for _ in 0..n {
        plaintext.push(rng.gen());
    }

    // pick ECB or CBC
    if rng.gen() {
        return (Mode::ECB, aes_ecb_encrypt(&key, &plaintext));
    }

    let mut iv = [0; 16];
    rng.fill_bytes(&mut iv);
    return (Mode::CBC, aes_cbc_encrypt(&key, &iv, &plaintext));
}
