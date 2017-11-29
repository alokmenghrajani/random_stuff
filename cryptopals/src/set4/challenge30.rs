/**
 * Break an MD4 keyed MAC using length extension
 * Second verse, same as the first, but use MD4 instead of SHA-1. Having done this attack once
 * against SHA-1, the MD4 variant should take much less time; mostly just the time you'll spend
 * Googling for an implementation of MD4.
 *
 * You're thinking, why did we bother with this?
 * Blame Stripe. In their second CTF game, the second-to-last challenge involved breaking an
 * H(k, m) MAC with SHA1. Which meant that SHA1 code was floating all over the Internet. MD4 code,
 * not so much.
 */

use utils::hex::hex_encode;
use utils::hex::hex_decode;
use utils::md4::{Md4, Digest};
use digest::{Input, FixedOutput};

use byteorder::{ByteOrder, LittleEndian};

pub fn run() {
    // ensure md4 works
    let m = "The quick brown fox jumps over the lazy dog".as_bytes();
    let mut md4 = Md4::new();
    md4.process(m);
    let r = hex_encode(&md4.fixed_result());
    assert_eq!(r, "1bee69a46ba811185c194762abaeae90");

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
        let mut state = [0; 4];
        LittleEndian::read_u32_into(&mac, &mut state);
        let mut md4 = Md4::new_with_state(state, (admin.len() + prefix_len) as u64);
        md4.process(";admin=true".as_bytes());
        let forged_mac = &md4.fixed_result();
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
    println!("checking Md4::new_without_padding works:");

    let s = "hello world".as_bytes();
    let t = pad(s, 0);
    println!("  {:?}", t);

    let mut md4 = Md4::new();
    md4.process(s);
    let hash1 = hex_encode(&md4.fixed_result());
    println!("  hash1: {:?}", hash1);

    let mut md4 = Md4::new_without_padding();
    md4.process(&t);
    let hash2 = hex_encode(&md4.fixed_result());
    println!("  hash2: {:?}", hash2);
    assert_eq!(hash1, hash2);
}

fn new_with_state() {
    println!("checking Md4::new_with_state works:");

    let md4 = Md4::new_without_padding();
    let initial_state = &md4.fixed_result();
    println!("  {:?}", initial_state);
    let mut state = [0; 4];
    LittleEndian::read_u32_into(initial_state, &mut state);
    let mut md4 = Md4::new_with_state(state, 0);
    md4.process("hello world".as_bytes());
    let hash = hex_encode(&md4.fixed_result());
    assert_eq!(hash, "aa010fbc1d14c795d86ef98c95479d17");
}

/**
 * pad a string in the same way as md4. The easiest way to test that this code is correct is to
 * compare the results between md4 and no_padding_md4.
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
    LittleEndian::write_u64(&mut buf, ((msg.len() + prefix) * 8) as u64);
    for i in 0..8 {
        r.push(buf[i]);
    }
    assert!(r.len() % 64 == 0);
    return r.into_iter().skip(prefix).collect();
}

pub fn gen_mac(message: &[u8]) -> Vec<u8> {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let mut md4 = Md4::new();
    md4.process(&key);
    md4.process(message);
    return md4.fixed_result().to_vec();
}

pub fn verify_mac(message: &[u8], mac: &[u8]) -> bool {
    let m = gen_mac(&message);
    return m == mac;
}
