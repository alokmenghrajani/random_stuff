/**
 * CTR bitflipping
 * There are people in the world that believe that CTR resists bit flipping attacks of the kind to
 * which CBC mode is susceptible.
 *
 * Re-implement the CBC bitflipping exercise from earlier to use CTR mode instead of CBC mode.
 * Inject an "admin=true" token.
 */

use utils::aes::aes_ctr;
use utils::hex::hex_decode;

pub fn run() {
    // we'll assume that we don't know where the ciphertext needs to be mutated and that the nonce
    // is always different. We perform 3 xors on the ciphertext at all possible offsets, until we
    // have the right string.
    let ciphertext = encrypt(String::from("XadminYtrueX"));
    println!("{:?}", ciphertext);

    for i in 0..(ciphertext.len() - 12) {
        let mut c = ciphertext.clone();
        c[i] = c[i] ^ ('X' as u8) ^ (';' as u8);
        c[i + 6] = c[i + 6] ^ ('Y' as u8) ^ ('=' as u8);
        c[i + 11] = c[i + 11] ^ ('X' as u8) ^ (';' as u8);
        let (ok, result) = decrypt(&c);
        println!("debug: {} {}", i, result);
        if ok {
            println!("debug: {} {}{}", i, " ".repeat(i + 1), "^".repeat(10));
            println!("Success!");
            return;
        }
    }
    println!("Failed.");
}

fn encrypt(input: String) -> Vec<u8> {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let input = input.replace(";", "%3b");
    let input = input.replace("=", "%3d");

    let mut buf: Vec<u8> = Vec::new();
    buf.extend("comment1=cooking%20MCs;userdata=".as_bytes());
    buf.extend(input.into_bytes());
    buf.extend(";comment2=%20like%20a%20pound%20of%20bacon".as_bytes());

    let nonce = [0; 8];
    return aes_ctr(&key, &nonce, &buf);
}

fn decrypt(input: &[u8]) -> (bool, String) {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let nonce = [0; 8];
    let plaintext = aes_ctr(&key, &nonce, &input);
    let s: String = plaintext.into_iter().map(|c| c as char).collect();
    return (s.contains(";admin=true;"), s);
}
