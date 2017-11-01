pub fn base64_encode(input: &[u8]) -> String {
    // the result is size() * 4 / 3 rounded to next multiple of 4.
    let final_size = (input.len() / 3 * 4 + 4 - 1) / 4 * 4;
    let mut r = String::with_capacity(final_size);

    // We populate an intermediate Vec<u8> where each u8 represents a bit.
    // Not the most efficient thing...
    let mut bits: Vec<u8> = Vec::with_capacity(input.len() * 8);
    for i in input {
        for j in (0..8).rev() {
            bits.push((((*i as i32) >> j) & 1) as u8);
        }
    }
    // Process the bits in chunks of 6 bits.
    for chunk in bits.chunks(6) {
        r.push(base64_encode_chunk(chunk));
    }
    // Append as many '=' characters as needed.
    while r.len() % 4 != 0 {
        r.push('=');
    }
    return r;
}

fn base64_encode_chunk(a: &[u8]) -> char {
    let mut n = 0;
    for i in a {
        n = (n << 1) | *i as i32;
    }
    // the last chunk can be less than 6 bits
    let missing = 6 - a.len();
    n = n << missing;

    let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    return b64[n as usize] as char;
}

pub fn base64_decode(input: &str) -> Vec<u8> {
    let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = Vec::with_capacity(input.len() / 4 * 3);
    for i in input.bytes() {
        // for now, we'll just ignore "=".
        if i == ('=' as u8) {
            continue;
        }
        match b64.find(i as char) {
            Some(n) => {
                for i in (0..6).rev() {
                    r.push((((n as i32) >> i) & 1) as u8);
                }
            }
            None => println!("Invalid base64 char: {}", i as char),
        }
    }
    // read the bits, 8 at a time and convert to u8. Discard any left-over.
    let b = r.chunks(8)
        .filter(|x| x.len() == 8)
        .map(|x| {
            let mut n = 0;
            for i in x {
                n = (n << 1) | *i as i32
            }
            return n as u8;
        })
        .collect();
    return b;
}
