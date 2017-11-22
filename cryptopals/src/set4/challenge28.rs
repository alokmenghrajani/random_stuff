/**
 * Implement a SHA-1 keyed MAC
 * Find a SHA-1 implementation in the language you code in.
 *
 * Don't cheat. It won't work.
 * Do not use the SHA-1 implementation your language already provides (for instance, don't use the
 * "Digest" library in Ruby, or call OpenSSL; in Ruby, you'd want a pure-Ruby SHA-1).
 * Write a function to authenticate a message under a secret key by using a secret-prefix MAC,
 * which is simply:
 *
 * SHA1(key || message)
 * Verify that you cannot tamper with the message without breaking the MAC you've produced, and
 * that you can't produce a new MAC without knowing the secret key.
 */

use utils::hex::hex_decode;
use utils::hex::hex_encode;
use utils::sha1::Sha1;

pub fn run() {
    let mut sha1 = Sha1::new();
    sha1.update("hello world".as_bytes());
    let hash = hex_encode(&sha1.digest().bytes());
    assert_eq!(hash, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");

    let mac1 = gen_mac(&"hello world".as_bytes());
    let mac2 = gen_mac(&"hello porld".as_bytes());
    println!("{}", hex_encode(&mac1));
    println!("{}", hex_encode(&mac2));

    assert!(verify_mac(&"hello world".as_bytes(), &mac1));
    assert!(!verify_mac(&"hello world".as_bytes(), &mac2));
    assert!(verify_mac(&"hello porld".as_bytes(), &mac2));
    assert!(!verify_mac(&"hello porld".as_bytes(), &mac1));
}

fn gen_mac(message: &[u8]) -> Vec<u8> {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let mut sha1 = Sha1::new();
    sha1.update(&key);
    sha1.update(message);
    return sha1.digest().bytes().to_vec();
}

fn verify_mac(message: &[u8], mac: &[u8]) -> bool {
    let m = gen_mac(&message);
    return m == mac;
}
