/**
 * Converts a hex string to Vec<u8> and vice-versa.
 */
pub fn hex_encode(input: &[u8]) -> String {
    // the result of hex encoding is twice the size
    let mut r = String::with_capacity(input.len() * 2);
    for i in input {
        r.push(hex_encode_char(i >> 4));
        r.push(hex_encode_char(i & 0xf));
    }
    return r;
}

fn hex_encode_char(input: u8) -> char {
    let map = "0123456789abcdef".as_bytes();
    return map[input as usize] as char;
}

pub fn hex_decode(input: &str) -> Vec<u8> {
    // the result of decoding the string is half the size
    if input.len() % 2 != 0 {
        panic!("input length should be a multiple of 2: {}", input)
    }
    let mut r = Vec::with_capacity(input.len() / 2);

    // read 2 bytes of input at a time
    let chars = input.as_bytes().chunks(2);
    for pair in chars {
        r.push(hex_decode_char(pair[0]) * 16 + hex_decode_char(pair[1]));
    }
    return r;
}

fn hex_decode_char(input: u8) -> u8 {
    let map = "0123456789abcdef";
    return match map.find(input as char) {
        Some(n) => n as u8,
        None => panic!("Invalid hex char: {}", input as char),
    };
}
