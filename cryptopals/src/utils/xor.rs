pub fn xor(input1: &[u8], input2: &[u8]) -> Vec<u8> {
    let mut r = Vec::with_capacity(input1.len());
    for i in 0..input1.len() {
        r.push(input1[i] ^ input2[i % input2.len()]);
    }
    return r;
}
