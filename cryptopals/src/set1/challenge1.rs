/**
 * Convert hex to base64
 * The string:
 * 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
 *
 * Should produce:
 * SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
 *
 * So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
 */
use utils::hex::hex_decode;
use utils::base64::base64_encode;

pub fn run() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let a = hex_decode(hex);
    let b = base64_encode(&a);
    println!("hex to base64: {}", b);
    assert_eq!(b,
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    let hex = "1122";
    let a = hex_decode(hex);
    let b = base64_encode(&a);
    println!("hex to base64: {}", b);
    assert_eq!(b, "ESI=");

    let hex = "aabbccdd";
    let a = hex_decode(hex);
    let b = base64_encode(&a);
    println!("hex to base64: {}", b);
    assert_eq!(b, "qrvM3Q==");
}
