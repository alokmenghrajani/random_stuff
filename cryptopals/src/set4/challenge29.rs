/**
 * Break a SHA-1 keyed MAC using length extension
 * Secret-prefix SHA-1 MACs are trivially breakable.
 *
 * The attack on secret-prefix SHA1 relies on the fact that you can take the ouput of SHA-1 and use
 * it as a new starting point for SHA-1, thus taking an arbitrary SHA-1 hash and "feeding it more
 * data".
 *
 * Since the key precedes the data in secret-prefix, any additional data you feed the SHA-1 hash in
 * this fashion will appear to have been hashed with the secret key.
 *
 * To carry out the attack, you'll need to account for the fact that SHA-1 is "padded" with the
 * bit-length of the message; your forged message will need to include that padding. We call this
 * "glue padding". The final message you actually forge will be:
 *
 * SHA1(key || original-message || glue-padding || new-message)
 * (where the final padding on the whole constructed message is implied)
 *
 * Note that to generate the glue padding, you'll need to know the original bit length of the
 * message; the message itself is known to the attacker, but the secret key isn't, so you'll need
 * to guess at it.
 *
 * This sounds more complicated than it is in practice.
 *
 * To implement the attack, first write the function that computes the MD padding of an arbitrary
 * message and verify that you're generating the same padding that your SHA-1 implementation is
 * using. This should take you 5-10 minutes.
 *
 * Now, take the SHA-1 secret-prefix MAC of the message you want to forge --- this is just a SHA-1
 * hash --- and break it into 32 bit SHA-1 registers (SHA-1 calls them "a", "b", "c", &c).
 *
 * Modify your SHA-1 implementation so that callers can pass in new values for "a", "b", "c" &c
 * (they normally start at magic numbers). With the registers "fixated", hash the additional data
 * you want to forge.
 *
 * Using this attack, generate a secret-prefix MAC under a secret key (choose a random word from
 * /usr/share/dict/words or something) of the string:
 *
 * "comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon"
 * Forge a variant of this message that ends with ";admin=true".
 *
 * This is a very useful attack.
 * For instance: Thai Duong and Juliano Rizzo, who got to this attack before we did, used it to
 * break the Flickr API.
 */

use utils::hex::hex_encode;
use utils::sha1::Sha1;
use set4::challenge28::gen_mac;
use set4::challenge28::verify_mac;

use byteorder::{ByteOrder, BigEndian};

pub fn run() {
    new_without_padding();

    new_with_state();

    // everything togther
    println!("generating user mac:");
    let user = "comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon"
        .as_bytes();
    let mac = gen_mac(user);
    println!("  user mac: {:?}", mac);

    println!("generating forged_mac:");
    for prefix_len in 0..64 {
        let mut admin = Vec::new();
        admin.extend_from_slice(&pad(user, prefix_len));
        let mut state = [0; 5];
        BigEndian::read_u32_into(&mac, &mut state);
        let mut sha1 = Sha1::new_with_state(state, (admin.len() + prefix_len) as u64);
        sha1.update(";admin=true".as_bytes());
        let forged_mac = &sha1.digest().bytes();
        println!("  forged mac: {:?}", forged_mac);

        println!("verifying:");
        admin.extend_from_slice(";admin=true".as_bytes());
        println!("  admin: {:?}", admin);
        let ok = verify_mac(&admin, forged_mac);
        println!("  result: {} => {}", prefix_len, ok);
        if ok {
            break;
        }
    }
}

fn new_without_padding() {
    println!("checking Sha1::new_without_padding works:");

    let s = "hello world".as_bytes();
    let t = pad(s, 0);
    println!("  {:?}", t);

    let mut sha1 = Sha1::new();
    sha1.update(s);
    let hash1 = hex_encode(&sha1.digest().bytes());
    println!("  hash1: {:?}", hash1);

    let mut sha1 = Sha1::new_without_padding();
    sha1.update(&t);
    let hash2 = hex_encode(&sha1.digest().bytes());
    println!("  hash2: {:?}", hash2);
    assert_eq!(hash1, hash2);
}

fn new_with_state() {
    println!("checking Sha1::new_with_state works:");

    let sha1 = Sha1::new_without_padding();
    let initial_state = &sha1.digest().bytes();
    println!("  {:?}", initial_state);
    let mut state = [0; 5];
    BigEndian::read_u32_into(initial_state, &mut state);
    let mut sha1 = Sha1::new_with_state(state, 0);
    sha1.update("hello world".as_bytes());
    let hash = hex_encode(&sha1.digest().bytes());
    assert_eq!(hash, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
}

/**
 * pad a string in the same way as sha1. The easiest way to test that this code is correct is to
 * compare the results between sha1 and no_padding_sha1.
 */
fn pad(msg: &[u8], prefix: usize) -> Vec<u8> {
    let mut r = Vec::new();
    for _ in 0..prefix {
        r.push(0x00);
    }
    for i in 0..msg.len() {
        r.push(msg[i]);
    }
    r.push(0x80); // the message is always 8 bit aligned, so we can push 0x80.
    while r.len() % 64 != 56 {
        r.push(0x00);
    }
    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, ((msg.len() + prefix) * 8) as u64);
    for i in 0..8 {
        r.push(buf[i]);
    }
    assert!(r.len() % 64 == 0);
    return r.into_iter().skip(prefix).collect();
}
