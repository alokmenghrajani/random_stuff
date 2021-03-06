/**
 * Single-byte XOR cipher
 * The hex encoded string:
 *
 * 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
 * ... has been XOR'd against a single character. Find the key, decrypt the message.
 *
 * You can do this by hand. But don't: write code to do it for you.
 *
 * How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a
 * good metric. Evaluate each output and choose the one with the best score.
 */
use utils::hex::hex_decode;
use utils::xor::xor;

pub fn run() {
    let input = hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let mut best = (0, Vec::new());
    for i in 0..256 {
        let a = xor(&input, &[i as u8]);
        let b = ascii_score(&a);
        if b > best.0 {
            best = (b, a);
        }
    }
    println!("{:?}", best.1);
    let b = String::from_utf8(best.1).unwrap();
    println!("{}", b);
}

pub fn ascii_score(input: &Vec<u8>) -> u64 {
    return input.iter().fold(0, |acc, x| if *x == 32 ||
                                         (((*x >= 65) && (*x <= 90)) ||
                                          ((*x >= 97) && (*x <= 122))) {
        return acc + 1;
    } else {
        return acc;
    });
}
