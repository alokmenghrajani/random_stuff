/**
 * Single-byte XOR cipher
 * The hex encoded string:
 *
 * 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
 * ... has been XOR'd against a single character. Find the key, decrypt the message.
 *
 * You can do this by hand. But don't: write code to do it for you.
 *
 * How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
 */
use set1::challenge1::unhex;

pub fn run() {
    let input = unhex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let mut best = (0, Vec::new());
    for i in 0..256 {
        let a = single_byte_xor(&input, i as u8);
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
    return input.iter().fold(0, |acc, x| if *x == 32 || ((*x >= 65) && (*x <= 122)) {
        return acc + 1;
    } else {
        return acc;
    });
}

pub fn single_byte_xor(input1: &Vec<u8>, input2: u8) -> Vec<u8> {
    return input1.iter().map(|x| x ^ input2).collect();
}
