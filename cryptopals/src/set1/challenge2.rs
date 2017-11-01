/**
 * Fixed XOR
 * Write a function that takes two equal-length buffers and produces their XOR combination.
 *
 * If your function works properly, then when you feed it the string:
 *
 * 1c0111001f010100061a024b53535009181c
 * ... after hex decoding, and when XOR'd against:
 *
 * 686974207468652062756c6c277320657965
 * ... should produce:
 *
 * 746865206b696420646f6e277420706c6179
 */
use utils::hex::hex_decode;
use utils::hex::hex_encode;

pub fn run() {
    let input1 = hex_decode("1c0111001f010100061a024b53535009181c");
    let input2 = hex_decode("686974207468652062756c6c277320657965");
    let a = xor(&input1, &input2);
    let b = hex_encode(&a);
    println!("xor result: {}", b);
    assert_eq!(b, "746865206b696420646f6e277420706c6179");

}

fn xor(input1: &Vec<u8>, input2: &Vec<u8>) -> Vec<u8> {
    if input1.len() != input2.len() {
        panic!("inputs don't have same length: {} != {}",
               input1.len(),
               input2.len())
    }
    let mut r = Vec::new();
    for i in 0..input1.len() {
        r.push(input1[i] ^ input2[i]);
    }
    return r;
}
