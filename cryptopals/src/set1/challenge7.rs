/**
 * AES in ECB mode
 * The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode
 * under the key
 *
 * "YELLOW SUBMARINE".
 * (case-sensitive, without the quotes; exactly 16 characters; I like "YELLOW SUBMARINE"
 * because it's exactly 16 bytes long, and now you do too).
 *
 * Decrypt it. You know the key, after all.
 *
 * Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.
 */


use utils::base64::base64_decode;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use crypto::aes;
use crypto::blockmodes;
use crypto::buffer;
use crypto::buffer::WriteBuffer;
use crypto::buffer::ReadBuffer;
use crypto::buffer::BufferResult;

pub fn run() {
    // load lines from 7.txt
    let file = File::open("./src/set1/7.txt").expect("Unable to open 7.txt");
    let lines = BufReader::new(file).lines();
    let contents: String = lines.map(|x| x.unwrap()).collect();
    let data = base64_decode(contents.as_str());

    println!("{:?}", data);

    let key = "YELLOW SUBMARINE".as_bytes();

    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, key, blockmodes::PkcsPadding);

    let mut read_buffer = buffer::RefReadBuffer::new(&data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    let mut final_result = Vec::<u8>::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i)); //.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    let plaintext: String = final_result.into_iter().map(|x| x as char).collect();
    println!("{}", plaintext);
}
