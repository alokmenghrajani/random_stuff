/**
 * ECB cut-and-paste
 * Write a k=v parsing routine, as if for a structured cookie. The routine should take:
 *
 * foo=bar&baz=qux&zap=zazzle
 * ... and produce:
 *
 * {
 *   foo: 'bar',
 *   baz: 'qux',
 *   zap: 'zazzle'
 * }
 * (you know, the object; I don't care if you convert it to JSON).
 *
 * Now write a function that encodes a user profile in that format, given an email address. You
 * should have something like:
 *
 * profile_for("foo@bar.com")
 * ... and it should produce:
 *
 * {
 *   email: 'foo@bar.com',
 *   uid: 10,
 *   role: 'user'
 * }
 * ... encoded as:
 *
 * email=foo@bar.com&uid=10&role=user
 * Your "profile_for" function should not allow encoding metacharacters (& and =). Eat them, quote
 * them, whatever you want to do, but don't let people set their email address to
 * "foo@bar.com&role=admin".
 *
 * Now, two more easy functions. Generate a random AES key, then:
 *
 * Encrypt the encoded user profile under the key; "provide" that to the "attacker".
 * Decrypt the encoded user profile and parse it.
 * Using only the user input to profile_for() (as an oracle to generate "valid" ciphertexts) and
 * the ciphertexts themselves, make a role=admin profile.
 */

use utils::aes::aes_ecb_encrypt;
use utils::aes::aes_ecb_decrypt;
use utils::hex::hex_decode;
use utils::hex::hex_encode;

use std::collections::HashSet;

pub fn run() {
    let t = from_string(String::from("foo=bar&baz=qux&zap=zazzle"));
    println!("{:?}", t);
    let t = profile_for(String::from("foo@bar.com"));
    println!("profile: {:?}", t);

    let t = profile_for_encrypted(String::from("foo@bar.com"));
    println!("encrypted: {}", t);

    let t = decrypt(t);
    println!("decrypted: {}", t);

    // build 3 strings, then pick the right chunks
    let p1 = hex_decode(&profile_for_encrypted(String::from("xxxxxxxxxxadmin")));
    let p2 = hex_decode(&profile_for_encrypted(String::from("xxxxxxxxxxxadmin")));
    let p3 = hex_decode(&profile_for_encrypted(String::from("xxxxxxxxxxxxx")));

    let mut r: Vec<u8> = Vec::new();
    let p3 = p3.chunks(16).take(2);
    let p1 = p1.chunks(16).skip(1).take(1);
    let p2 = p2.chunks(16).skip(2).take(1);
    for i in p3.chain(p1).chain(p2) {
        r.extend(i);
    }
    let r = hex_encode(&r);
    let r = decrypt(r);
    println!("decrypted: {}", r);
}

fn decrypt(input: String) -> String {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let r: String = aes_ecb_decrypt(&key, &hex_decode(&input)).iter().map(|c| *c as char).collect();
    return r;
}

fn profile_for_encrypted(input: String) -> String {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let t = profile_for(input);
    return hex_encode(&aes_ecb_encrypt(&key, t.as_bytes()));
}

fn profile_for(input: String) -> String {
    let mut r = Vec::new();
    // disallow = and &
    let input = input.replace(|c| c == '=' || c == '&', "_");
    r.push((String::from("email"), input));
    r.push((String::from("uid"), String::from("10")));
    r.push((String::from("role"), String::from("user")));
    return to_string(r);
}

fn to_string(input: Vec<(String, String)>) -> String {
    let t: Vec<String> = input.iter().map(|e| format!("{}={}", e.0, e.1)).collect();
    return t.join("&");
}

fn from_string(input: String) -> Vec<(String, String)> {
    let mut r = Vec::new();
    let mut keys = HashSet::new();
    for pair in input.split('&') {
        let v: Vec<&str> = pair.split('=').collect();
        if v.len() != 2 {
            println!("from_string: invalid pair: {}", pair);
            continue;
        }
        let k = String::from(v[0]);
        if keys.contains(&k) {
            println!("from_string: duplicate key: {}", pair);
            continue;
        }
        keys.insert(k.clone());
        r.push((k, String::from(v[1])));
    }
    return r;
}
