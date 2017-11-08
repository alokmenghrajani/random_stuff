/**
 * CBC bitflipping attacks
 * Generate a random AES key.
 *
 * Combine your padding code and CBC code to write two functions.
 *
 * The first function should take an arbitrary input string, prepend the string:
 *
 * "comment1=cooking%20MCs;userdata="
 * .. and append the string:
 *
 * ";comment2=%20like%20a%20pound%20of%20bacon"
 * The function should quote out the ";" and "=" characters.
 *
 * The function should then pad out the input to the 16-byte AES block length and encrypt it under
 * the random AES key.
 *
 * The second function should decrypt the string and look for the characters ";admin=true;" (or,
 * equivalently, decrypt, split the string on ";", convert each resulting string into 2-tuples, and
 * look for the "admin" tuple).
 *
 * Return true or false based on whether the string exists.
 *
 * If you've written the first function properly, it should not be possible to provide user input
 * to it that will generate the string the second function is looking for. We'll have to break the
 * crypto to do that.
 *
 * Instead, modify the ciphertext (without knowledge of the AES key) to accomplish this.
 *
 * You're relying on the fact that in CBC mode, a 1-bit error in a ciphertext block:
 * - Completely scrambles the block the error occurs in
 * - Produces the identical 1-bit error(/edit) in the next ciphertext block.
 */

use utils::aes::aes_cbc_decrypt;
use utils::aes::aes_cbc_encrypt;
use utils::hex::hex_decode;
use utils::hex::hex_encode;
use utils::pkcs7::pkcs7_unpad;

pub fn run() {
    let ciphertext = encrypt(String::from("XadminYtrueX"));
    println!("{:?}", ciphertext);

    let plaintext = decrypt(ciphertext.clone());
    println!("{:?}", plaintext);

    // we have the following ciphertext:
    // 47db33170edbe00a8842f7f682632b466aee14279b494457aab6be7b8b477d2d0f2137e807c03ca47f68de623be7f43019dbdf0dff401b057dffc5a2239b4407b6059c86d017ecea85992a049895d50d996f54052a6c6b69c09c1a442a69647f
    //
    // and we want to flip bits in:
    // 1234567890123456
    //                 1234567890123456
    //                                 1234567890123456
    // comment1=cooking%20MCs;userdata=XadminYtrueX;comment2=%20like%20a%20pound%20of%20bacon

    // so we want to flip bytes 1, 7 and 12 in block 2.
    let mut c = hex_decode(&ciphertext);
    let mut c: Vec<&mut [u8]> = c.chunks_mut(16).collect();
    println!("before: {:?}", c);
    c[1][0] = c[1][0] ^ ('X' as u8) ^ (';' as u8);
    c[1][6] = c[1][6] ^ ('Y' as u8) ^ ('=' as u8);
    c[1][11] = c[1][11] ^ ('X' as u8) ^ (';' as u8);
    println!("after:  {:?}", c);

    let new_ciphertext = hex_encode(&flatten(c));
    let new_plaintext = decrypt(new_ciphertext);
    println!("{:?}", new_plaintext);
}

fn flatten(input: Vec<&mut [u8]>) -> Vec<u8> {
    let mut r = Vec::new();
    for i in input {
        r.extend_from_slice(i);
    }
    return r;
}


fn encrypt(input: String) -> String {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let input = input.replace(";", "%3b");
    let input = input.replace("=", "%3d");

    let mut buf: Vec<u8> = Vec::new();
    buf.extend("comment1=cooking%20MCs;userdata=".as_bytes());
    buf.extend(input.into_bytes());
    buf.extend(";comment2=%20like%20a%20pound%20of%20bacon".as_bytes());

    let iv = [0; 16];
    let ciphertext = aes_cbc_encrypt(&iv, &key, &buf);
    return hex_encode(&ciphertext);
}

fn decrypt(input: String) -> bool {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let ciphertext = hex_decode(&input);
    let iv = [0; 16];
    let plaintext = aes_cbc_decrypt(&iv, &key, &ciphertext);
    match pkcs7_unpad(&plaintext) {
        Some(p) => {
            let s: String = p.into_iter().map(|c| c as char).collect();
            println!("debug: {}", s);
            return s.contains(";admin=true;");
        }
        None => {
            println!("Bad padding!");
            return false;
        }
    };
}
