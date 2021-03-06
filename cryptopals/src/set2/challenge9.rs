/**
 * Implement PKCS#7 padding
 * A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into
 * ciphertext. But we almost never want to transform a single block; we encrypt irregularly-sized
 * messages.
 *
 * One way we account for irregularly-sized messages is by padding, creating a plaintext that is an
 * even multiple of the blocksize. The most popular padding scheme is called PKCS#7.
 *
 * So: pad any block to a specific block length, by appending the number of bytes of padding to the
 * end of the block. For instance,
 *
 * "YELLOW SUBMARINE"
 * ... padded to 20 bytes would be:
 *
 * "YELLOW SUBMARINE\x04\x04\x04\x04"
 */

use utils::hex::hex_encode;
use utils::pkcs7::pkcs7_pad;

pub fn run() {
    let input = "YELLOW SUBMARINE";
    let padded_input = pkcs7_pad(input.as_bytes(), 20);
    let t = hex_encode(&padded_input);
    println!("{}", t);
    assert_eq!(t, "59454c4c4f57205355424d4152494e4504040404");

    let input = "FOO";
    let padded_input = pkcs7_pad(input.as_bytes(), 3);
    let t = hex_encode(&padded_input);
    println!("{}", t);
    assert_eq!(t, "464f4f030303");

    let input = "FOO";
    let padded_input = pkcs7_pad(input.as_bytes(), 4);
    let t = hex_encode(&padded_input);
    println!("{}", t);
    assert_eq!(t, "464f4f01");
}
