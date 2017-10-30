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
pub fn run() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let a = unhex(hex);
    let b = base64_encode(&a);
    println!("hex to base64: {}", b);
    assert_eq!(b,
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    let hex = "1122";
    let a = unhex(hex);
    let b = base64_encode(&a);
    println!("hex to base64: {}", b);
    assert_eq!(b, "ESI=");

    let hex = "aabbccdd";
    let a = unhex(hex);
    let b = base64_encode(&a);
    println!("hex to base64: {}", b);
    assert_eq!(b, "qrvM3Q==");
}

fn base64_encode(input: &Vec<u8>) -> String {
    let mut r = String::new();
    // we read (upto) 6 bits at a time and emit one char.
    let mut b: Vec<Vec<u8>> = input.iter().map(|x| byte_to_bits(x)).collect();
    let mut c: Vec<u8> = flatten(&mut b);

    loop {
        if c.len() < 6 {
            break;
        }
        let d = c.split_off(6);
        r.push(bits_to_b64(c));
        c = d;
    }
    if c.len() > 0 {
        // handle the case where we have less than 6 bits remaining
        while c.len() < 6 {
            // pad to 6 bits
            c.push(0);
        }
        r.push(bits_to_b64(c));
    }
    while r.len() % 4 != 0 {
        // append the '=' character at the end, if needed.
        r.push('=');
    }
    return r;
}

fn flatten(input: &mut Vec<Vec<u8>>) -> Vec<u8> {
    let mut r = Vec::new();
    for i in input {
        r.append(i);
    }
    return r;
}

fn byte_to_bits(a: &u8) -> Vec<u8> {
    let mut r = Vec::new();
    for i in (0..8).rev() {
        r.push((((*a as i32) >> i) & 1) as u8);
    }
    return r;
}

fn bits_to_b64(a: Vec<u8>) -> char {
    let mut n = 0;
    for i in a {
        n = (n << 1) | i as i32
    }
    let mut b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars();
    return b64.nth(n as usize).unwrap();
}

fn unhex(input: &str) -> Vec<u8> {
    let mut r = Vec::new();
    if input.len() % 2 != 0 {
        panic!("input length should be a multiple of 2: {}", input)
    }
    // read 2 bytes of inputs at a time
    let mut chars = input.as_bytes().iter();
    loop {
        match (chars.next(), chars.next()) {
            (Some(a), Some(b)) => {
                r.push(unhex_char(a) * 16 + unhex_char(b));
            }
            _ => break,
        }
    }
    return r;
}

// TODO: not sure if unhex_char should be &u8 or u8?
fn unhex_char(a: &u8) -> u8 {
    if (*a >= ('0' as u8)) && (*a <= ('9' as u8)) {
        return a - ('0' as u8);
    }
    if (*a >= ('a' as u8)) && (*a <= ('f' as u8)) {
        return a - ('a' as u8) + 10;
    }
    panic!("invalid hex character: {}", a);
}
