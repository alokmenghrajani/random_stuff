/**
 * Recover the key from CBC with IV=Key
 * Take your code from the CBC exercise and modify it so that it repurposes the key for CBC
 * encryption as the IV.
 *
 * Applications sometimes use the key as an IV on the auspices that both the sender and the
 * receiver have to know the key already, and can save some space by using it as both a key and an
 * IV.
 *
 * Using the key as an IV is insecure; an attacker that can modify ciphertext in flight can get the
 * receiver to decrypt a value that will reveal the key.
 *
 * The CBC code from exercise 16 encrypts a URL string. Verify each byte of the plaintext for ASCII
 * compliance (ie, look for high-ASCII values). Noncompliant messages should raise an exception or
 * return an error that includes the decrypted plaintext (this happens all the time in real
 * systems, for what it's worth).
 *
 * Use your code to encrypt a message that is at least 3 blocks long:
 *
 * AES-CBC(P_1, P_2, P_3) -> C_1, C_2, C_3
 * Modify the message (you are now the attacker):
 *
 * C_1, C_2, C_3 -> C_1, 0, C_1
 *
 * Decrypt the message (you are now the receiver) and raise the appropriate error if high-ASCII is
 * found.
 *
 * As the attacker, recovering the plaintext from the error, extract the key:
 *
 * P'_1 XOR P'_3
 */

use utils::aes::aes_cbc_decrypt;
use utils::aes::aes_cbc_encrypt;
use utils::hex::hex_decode;
use utils::hex::hex_encode;
use utils::pkcs7::pkcs7_unpad;
use utils::xor::xor;

pub fn run() {
    let ciphertext = encrypt(String::from("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbb"));
    println!("{:?}", ciphertext);

    // create a ciphertext which contains block0, block0, block0.
    // bruteforce the last byte until the padding check passes.
    for padding in 0..256 {
        let mut c = Vec::new();
        for i in 0..16 {
            c.push(ciphertext[i]);
        }
        for i in 0..16 {
            c.push(ciphertext[i]);
        }
        for i in 0..16 {
            c.push(ciphertext[i]);
        }
        c[47] = padding as u8;
        let (err, plaintext) = decrypt(&c);
        if err == "bad padding" {
            continue;
        } else if err == "high bytes" {
            println!("{:?}", c);
            // we found a valid padding!
            // the key is plaintext0 xor plaintext1 xor ciphertext0
            let p0 = &plaintext[0..16];
            let p1 = &plaintext[16..32];
            let c0 = &ciphertext[0..16];
            let key = xor(&xor(&p0, &p1), &c0);
            let key = hex_encode(&key);
            println!("key: {}", key);
            assert_eq!(key, "cf358337dd4dfc1ddb710e30a1809e3f");
            break;
        } else {
            assert!(false);
        }
    }
}

fn encrypt(input: String) -> Vec<u8> {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let input = input.replace(";", "%3b");
    let input = input.replace("=", "%3d");

    let mut buf: Vec<u8> = Vec::new();
    buf.extend("comment1=cooking%20MCs;userdata=".as_bytes());
    buf.extend(input.into_bytes());
    buf.extend(";comment2=%20like%20a%20pound%20of%20bacon".as_bytes());

    return aes_cbc_encrypt(&key, &key, &buf);
}

fn decrypt(input: &[u8]) -> (&str, Vec<u8>) {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let plaintext = aes_cbc_decrypt(&key, &key, &input);
    match pkcs7_unpad(&plaintext) {
        Some(p) => {
            let high_bytes = p.iter().any(|c| *c > 127);
            if high_bytes {
                return ("high bytes", p);
            } else {
                return ("ok", Vec::new());
            }
        }
        None => {
            return ("bad padding", Vec::new());
        }
    };
}
