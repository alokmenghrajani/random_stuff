// This puzzle can be solved in many ways:
// - manually convert the short piece of input into your favorite language. It's quick if your
//   language supports GOTO-style statements. You can also use loops otherwise.
// - write an interpreter. Probably what most people did?
// - write a compiler / JIT. That's the crazy approach I decided to take :)
//
// I proceed in three steps:
// - parse the input and create a Vec<Cmd>.
// - write the x64 machine code into a buffer. I do this step twice because the machine code's
//   length is not known ahead of time and is needed to compute the offset of jumps.
// - jump to the start of the buffer and execute the input.

// I found a tutorial by Jonathan Turner, which was very helpful:
// http://www.jonathanturner.org/2015/12/building-a-simple-jit-in-rust.html

extern crate regex;
extern crate libc;

use self::regex::Regex;
use std::mem;
use std::ops::{Index, IndexMut};

pub fn solve(input: &str) {
    let test_input = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
    assert_eq!(_solve(test_input, (0, 0, 0, 0)), 42);

    println!("part 1: {}", _solve(input, (0, 0, 0, 0)));
    println!("part 2: {}", _solve(input, (0, 0, 1, 0)));
}

// Normally, compilers have multiple intermediate representations. We can take a shortcut here and
// use something x64 specific.
#[derive(Clone)]
enum Cmd {
    CpyI(i64, String),
    CpyR(String, String),
    IncR(String),
    DecR(String),
    JnzI(i64, isize),
    JnzR(String, isize),
    Nop,
}

fn i64_to_bytes(v: i64) -> Vec<u8> {
    vec![(v & 0xff) as u8,
         ((v >> 8) & 0xff) as u8,
         ((v >> 16) & 0xff) as u8,
         ((v >> 24) & 0xff) as u8]
}

fn reg_to_bits(r: String) -> u8 {
    let b = r.bytes().nth(0).unwrap();
    b - b'a' + 0xc0
}

// Converting instructions to machine code is a pain. Thankfully, we just care about a subet.
fn get_bytes(cmd: Cmd) -> Vec<u8> {
    let mut bytes = vec![];
    match cmd {
        Cmd::CpyI(i, r) => {
            bytes.extend_from_slice(&[0x49, 0xc7, 0xc0 | reg_to_bits(r)]);
            bytes.append(&mut i64_to_bytes(i));
        }
        Cmd::CpyR(src, dst) =>
            bytes.extend_from_slice(&[0x4d, 0x89, 0xc0 | (reg_to_bits(src) << 3) | reg_to_bits(dst)]),
        Cmd::IncR(r) => bytes.extend_from_slice(&[0x49, 0xff, 0xc0 | reg_to_bits(r)]),
        Cmd::DecR(r) => bytes.extend_from_slice(&[0x49, 0xff, 0xc8 | reg_to_bits(r)]),
        Cmd::JnzI(i, _) => {
            bytes.extend_from_slice(&[0x48, 0xc7, 0xc0]);
            bytes.append(&mut i64_to_bytes(i)); // mov rax, i
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x75, 0x00]); // jne offset
        }
        Cmd::JnzR(r, _) => {
            bytes.extend_from_slice(&[0x4c, 0x89, (0xc0 | reg_to_bits(r) << 3)]); // mov rax, r
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x75, 0x00]); // jne offset
        }
        Cmd::Nop => bytes.push(0x90),
    }
    bytes
}

// Converting instructions to machine code is a pain. Thankfully, we just care about a subet.
fn get_bytes2(cmd: Cmd, offset: usize, offsets: &Vec<(Cmd, usize)>) -> Vec<u8> {
    let mut bytes = vec![];
    match cmd {
        Cmd::JnzI(i, o) => {
            let mut delta: isize = offsets[(offset as isize + o - 1) as usize].1 as isize;
            delta -= offsets[offset].1 as isize;
            assert_eq!(delta, (delta as i8) as isize);
            bytes.extend_from_slice(&[0x48, 0xc7, 0xc0]);
            bytes.append(&mut i64_to_bytes(i)); // mov rax, i
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x75, delta as u8]); // jne offset
        }
        Cmd::JnzR(r, o) => {
            let mut delta: isize = offsets[(offset as isize + o - 1) as usize].1 as isize;
            delta -= offsets[offset].1 as isize;
            assert_eq!(delta, (delta as i8) as isize);
            bytes.extend_from_slice(&[0x4c, 0x89, 0xc0 | (reg_to_bits(r.clone()) << 3)]); // mov rax, r
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x75, delta as u8]); // jne offset
        }
        _ => bytes.append(&mut get_bytes(cmd)),
    }
    bytes
}

fn _solve(input: &str, (reg_a, reg_b, reg_c, reg_d): (i64, i64, i64, i64)) -> i64 {
    // parse input
    let mut cmds = vec![];
    cmds.push(Cmd::CpyI(reg_a, "a".to_string()));
    cmds.push(Cmd::CpyI(reg_b, "b".to_string()));
    cmds.push(Cmd::CpyI(reg_c, "c".to_string()));
    cmds.push(Cmd::CpyI(reg_d, "d".to_string()));

    for line in input.trim().split('\n') {
        if let Some(cap) = Regex::new(r"^cpy (-?\d+) ([a-d])").unwrap().captures(line) {
            cmds.push(Cmd::CpyI(cap.at(1).unwrap().parse().unwrap(),
                                cap.at(2).unwrap().to_string()));
            continue;
        }
        if let Some(cap) = Regex::new(r"^cpy ([a-d]) ([a-d])").unwrap().captures(line) {
            cmds.push(Cmd::CpyR(cap.at(1).unwrap().to_string(),
                                cap.at(2).unwrap().to_string()));
            continue;
        }
        if let Some(cap) = Regex::new(r"^inc ([a-d])").unwrap().captures(line) {
            cmds.push(Cmd::IncR(cap.at(1).unwrap().to_string()));
            continue;
        }
        if let Some(cap) = Regex::new(r"^dec ([a-d])").unwrap().captures(line) {
            cmds.push(Cmd::DecR(cap.at(1).unwrap().to_string()));
            continue;
        }
        if let Some(cap) = Regex::new(r"^jnz (-?\d+) (-?\d+)").unwrap().captures(line) {
            cmds.push(Cmd::JnzI(cap.at(1).unwrap().parse().unwrap(),
                                cap.at(2).unwrap().parse().unwrap()));
            continue;
        }
        if let Some(cap) = Regex::new(r"^jnz ([a-d]) (-?\d+)").unwrap().captures(line) {
            cmds.push(Cmd::JnzR(cap.at(1).unwrap().to_string(),
                                cap.at(2).unwrap().parse().unwrap()));
            continue;
        }
        panic!("unknown command");
    }
    cmds.push(Cmd::Nop);

    // emit the machine code. We emit it twice so that we have all the right addresses for jumps.
    let mut cmds_offsets: Vec<(Cmd, usize)> = vec![];
    for cmd in cmds {
        cmds_offsets.push((cmd, 0));
    }
    let mut bytes = vec![];

    // We don't need to save any registers because we only use eax and r8-r11

    let mut cmd_offsets2 = vec![];
    for (cmd, _) in cmds_offsets {
        bytes.append(&mut get_bytes(cmd.clone()));
        cmd_offsets2.push((cmd, bytes.len()))
    }

    // second pass to resolve jumps
    bytes.truncate(0);
    for i in 0..cmd_offsets2.len() {
        let (cmd, _) = cmd_offsets2[i].clone();
        bytes.append(&mut get_bytes2(cmd, i, &cmd_offsets2));
    }

    // Since we don't save any registers, we don't need to restore any.
    bytes.extend_from_slice(&[0x4c, 0x89, 0xc0]); // mov rax, r8
    bytes.extend_from_slice(&[0xc3]); // ret

    // run the code
    let n_pages = (bytes.len() as f64 / PAGE_SIZE as f64).ceil() as usize;
    let mut jit: JitMemory = JitMemory::new(n_pages);
    for i in 0..bytes.len() {
        jit[i] = bytes[i];
    }
    let f: fn() -> i64 = unsafe { mem::transmute(jit.contents) };
    debug_me(f)
}

fn debug_me(fun: fn() -> i64) -> i64 {
    fun()
}

const PAGE_SIZE: usize = 4096;

struct JitMemory {
    contents: *mut u8,
}

impl JitMemory {
    fn new(num_pages: usize) -> JitMemory {
        let contents: *mut u8;
        unsafe {
            let size = num_pages * PAGE_SIZE;
            let mut _contents: *mut libc::c_void = mem::uninitialized();
            libc::posix_memalign(&mut _contents, PAGE_SIZE, size);
            libc::mprotect(_contents,
                           size,
                           libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);

            contents = mem::transmute(_contents);
        }

        JitMemory { contents: contents }
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.contents.offset(_index as isize) }
    }
}
