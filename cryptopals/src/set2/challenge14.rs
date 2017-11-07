/**
 * Byte-at-a-time ECB decryption (Harder)
 * Take your oracle function from #12. Now generate a random count of random bytes and prepend this
 * string to every plaintext. You are now doing:
 *
 * AES-128-ECB(random-prefix || attacker-controlled || target-bytes, random-key)
 * Same goal: decrypt the target-bytes.
 *
 * note: I'm going to assume the attacker doesn't know the block size or the random-prefix size
 *       range.
 */

use utils::aes::aes_ecb_encrypt;
use utils::hex::hex_decode;
use utils::base64::base64_decode;

use set2::challenge11::guess_mode;
use set2::challenge11::Mode;

use rand::Rng;
use rand;

pub fn run() {
    println!("guessing block size...");
    let block_size = guess_blocksize();
    println!("block size: {}", block_size);
    assert_eq!(block_size, 16);

    println!("guessing prefix padding...");
    let (prefix_padding, look_at) = guess_prefix_padding();
    println!("prefix_padding: {}, look_at: {}", prefix_padding, look_at);
    assert_eq!(prefix_padding, 7);
    assert_eq!(look_at, 2);

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
    let initial_size = oracle3(&buffer, prefix_padding, look_at).len();
    while oracle3(&buffer, prefix_padding, look_at).len() == initial_size {
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
            let temp = oracle3(&block1, prefix_padding, look_at);
            let target = temp.chunks(16).nth(block_to_break).unwrap();

            // iterate until we find the target
            let mut found = false;
            for c in 0..256 {
                block2[15] = c as u8;
                let t1 = oracle3(&block2, prefix_padding, look_at);
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

// We need to find a value k, such that {a x k}{b x 16}{b x 16} gives us two identical consecutive
// blocks. It turns out k and look_at can be used to compute the max random prefix length.
fn guess_prefix_padding() -> (usize, usize) {
    let mut look_at = 1;
    loop {
        for k in 0..16 {
            for _ in 0..100 {
                let mut buffer = Vec::new();
                for _ in 0..k {
                    buffer.push('a' as u8);
                }
                for _ in 0..32 {
                    buffer.push('b' as u8);
                }
                let t = oracle(&buffer);
                let chunks: Vec<&[u8]> = t.chunks(16).take(look_at + 1).collect();
                if chunks[look_at - 1] == chunks[look_at] {
                    return (k, look_at);
                }
            }
        }
        look_at += 1;
    }
}

// Similar code as in challenge 12, but we call oracle multiple times and keep the min length.
fn guess_blocksize() -> usize {
    let mut block_size = 0;
    let mut v = Vec::new();
    let mut prev = oracle2(&v, 100).len();
    loop {
        v.push('a' as u8);
        let t = oracle2(&v, 100).len();
        if t != prev {
            prev = t;
            break;
        }
    }
    block_size -= v.len() as isize;
    loop {
        v.push('a' as u8);
        let t = oracle2(&v, 100).len();
        if t != prev {
            break;
        }
    }
    block_size += v.len() as isize;
    return block_size as usize;
}

fn oracle3(input: &[u8], prefix_padding: usize, look_at: usize) -> Vec<u8> {
    loop {
        let mut buf = Vec::new();
        for _ in 0..prefix_padding {
            buf.push('a' as u8);
        }
        for _ in 0..32 {
            buf.push('b' as u8);
        }
        buf.extend(input);
        let t = oracle(&buf);
        let t2 = t.clone();
        let chunks: Vec<&[u8]> = t2.chunks(16).take(look_at + 1).collect();
        if chunks[look_at - 1] == chunks[look_at] {
            return t.into_iter().skip((look_at + 1) * 16).collect();
        }
    }
}

fn oracle2(input: &[u8], iterations: usize) -> Vec<u8> {
    let mut r = oracle(input);
    for _ in 1..iterations {
        let t = oracle(input);
        if r.len() < t.len() {
            r = t;
        }
    }
    return r;
}

fn oracle(input: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let mut plaintext = Vec::new();

    // prepend random bytes (5-10)
    let n = rng.next_u32() % 5 + 5;
    for _ in 0..n {
        plaintext.push(rng.gen());
    }

    plaintext.extend(input);
    plaintext.extend(base64_decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK"));

    return aes_ecb_encrypt(&key, &plaintext);
}

// fn pretty_print(input: &[u8]) {
//     for i in 0..input.len() {
//         if i % 16 == 0 {
//             println!("");
//         }
//         print!("{} ", input[i]);
//     }
//     println!("");
// }
