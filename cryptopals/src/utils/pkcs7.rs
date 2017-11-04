pub fn pkcs7_pad(input: &[u8], block_size: i32) -> Vec<u8> {
    let mut r = Vec::with_capacity(input.len() + block_size as usize);
    r.extend_from_slice(input);
    let remainder = block_size - (input.len() as i32 % block_size);
    for _ in 0..remainder {
        r.push(remainder as u8);
    }
    return r;
}
