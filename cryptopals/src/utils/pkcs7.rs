pub fn pkcs7_pad(input: &[u8], block_size: i32) -> Vec<u8> {
    let mut r = Vec::with_capacity(input.len() + block_size as usize);
    r.extend_from_slice(input);
    let remainder = block_size - (input.len() as i32 % block_size);
    for _ in 0..remainder {
        r.push(remainder as u8);
    }
    return r;
}

pub fn pkcs7_unpad(input: &[u8]) -> Option<Vec<u8>> {
    let last = input[input.len() - 1];
    for i in 1..last {
        if input[input.len() - i as usize - 1] != last {
            return None;
        }
    }
    let mut r = input.to_vec();
    r.truncate(input.len() - last as usize);
    return Some(r);
}
