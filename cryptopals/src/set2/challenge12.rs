/**
 * Byte-at-a-time ECB decryption (Simple)
 *
 * Copy your oracle function to a new function that encrypts buffers under ECB mode using a
 * consistent but unknown key (for instance, assign a single random key, once, to a global
 * variable).
 *
 * Now take that same function and have it append to the plaintext, BEFORE ENCRYPTING, the
 * following string:
 *
 * Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
 * aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
 * dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
 * YnkK
 *
 * Base64 decode the string before appending it. Do not base64 decode the string by hand; make your
 * code do it. The point is that you don't know its contents.
 *
 * What you have now is a function that produces:
 *
 * AES-128-ECB(your-string || unknown-string, random-key)
 *
 * It turns out: you can decrypt "unknown-string" with repeated calls to the oracle function!
 */

use utils::aes::aes_ecb_encrypt;
use utils::hex::hex_decode;
use utils::base64::base64_decode;

use set2::challenge11::guess_mode;
use set2::challenge11::Mode;

pub fn run() {
    // find block size
    let block_size = guess_blocksize();
    assert_eq!(block_size, 16);
    println!("block size: {}", block_size);

    // confirm ecb
    let mut v = Vec::new();
    for _ in 0..(3 * block_size) {
        v.push('a' as u8);
    }
    let mode = guess_mode(&oracle(&v));
    assert_eq!(mode, Mode::ECB);
    println!("mode: {:?}", mode);

    // ...u nkno wn## vs ...* unkn own#
    //    ^
    // ..un know n### vs ..u* unkn own#
    //    ^
    // .unk nown      vs .un* unkn own#
    //    ^
    // unkn own#      vs unk* unkn own#
    //    ^
    // ...u nkno wn## vs nkn* unkn own#
    //         ^
    // ..un know n### vs kno* unkn own#
    //         ^
    // .unk nown      vs now* unkn own#
    //         ^

    // calculate the plaintext size by seeing how many bytes we can add before we hit the next
    // block.
    let mut buffer = Vec::new();
    let initial_size = oracle(&buffer).len();
    while oracle(&buffer).len() == initial_size {
        buffer.push(0);
    }
    let plaintext_size = initial_size - buffer.len();
    println!("plaintext size: {}", plaintext_size);

    let mut plaintext = Vec::new();

    // this code is pretty ugly...
    let mut block2 = Vec::new();
    for _ in 0..16 {
        block2.push('a' as u8);
    }
    let mut block_to_break = 0;
    loop {
        for i in 0..16 {
            println!("char {} in block {}", i, block_to_break);
            let mut block1 = Vec::new();
            for _ in 0..(15 - i) {
                block1.push('a' as u8);
            }
            let temp = oracle(&block1);
            let target = temp.chunks(16).nth(block_to_break).unwrap();

            // iterate until we find the target
            let mut found = false;
            for c in 0..256 {
                block2[15] = c as u8;
                let t1 = oracle(&block2);
                let t2 = t1.chunks(16).nth(0).unwrap();
                if t2 == target {
                    found = true;
                    println!("  {} {}", c as u8 as char, c);
                    plaintext.push(c as u8);
                    if plaintext.len() == plaintext_size {
                        println!("");
                        let s: String = plaintext.into_iter().map(|x| x as char).collect();
                        println!("{}", s);
                        return;
                    }
                    block2.remove(0);
                    block2.push(0);
                    break;
                }
            }
            if !found {
                let s: String = plaintext.into_iter().map(|x| x as char).collect();
                println!("{}", s);
                panic!("search failed!");
            }
        }
        block_to_break += 1;
    }
}

fn guess_blocksize() -> usize {
    let mut block_size = 0;
    let mut v = Vec::new();
    let mut prev = oracle(&v).len();
    loop {
        v.push('a' as u8);
        let t = oracle(&v).len();
        if t != prev {
            prev = t;
            break;
        }
    }
    block_size -= v.len() as isize;
    loop {
        v.push('a' as u8);
        let t = oracle(&v).len();
        if t != prev {
            break;
        }
    }
    block_size += v.len() as isize;
    return block_size as usize;
}

fn oracle(input: &[u8]) -> Vec<u8> {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let mut plaintext = Vec::new();
    plaintext.extend(input);
    plaintext.extend(base64_decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK"));

    return aes_ecb_encrypt(&key, &plaintext);
}
