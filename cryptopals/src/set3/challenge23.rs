/**
 * Clone an MT19937 RNG from its output
 * The internal state of MT19937 consists of 624 32 bit integers.
 *
 * For each batch of 624 outputs, MT permutes that internal state. By permuting state regularly,
 * MT19937 achieves a period of 2**19937, which is Big.
 *
 * Each time MT19937 is tapped, an element of its internal state is subjected to a tempering
 * function that diffuses bits through the result.
 *
 * The tempering function is invertible; you can write an "untemper" function that takes an MT19937
 * output and transforms it back into the corresponding element of the MT19937 state array.
 *
 * To invert the temper transform, apply the inverse of each of the operations in the temper
 * transform in reverse order. There are two kinds of operations in the temper transform each
 * applied twice; one is an XOR against a right-shifted value, and the other is an XOR against a
 * left-shifted value AND'd with a magic number. So you'll need code to invert the "right" and the
 * "left" operation.
 *
 * Once you have "untemper" working, create a new MT19937 generator, tap it for 624 outputs,
 * untemper each of them to recreate the state of the generator, and splice that state into a new
 * instance of the MT19937 generator.
 *
 * The new "spliced" generator should predict the values of the original.
 */

use utils::mt_rand::MtRand;

use rand;
use rand::Rng;

const N: usize = 624;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;

pub fn run() {
    // pick a random seed
    let mut rng = rand::thread_rng();
    let seed = rng.next_u32();
    let mut mt = MtRand::new_with_seed(seed);
    println!("original seed: {}", seed);

    // unittests
    let y1 = rng.next_u32();
    let y2 = y1 ^ ((y1 >> U) & D);
    assert_eq!(to_string(new_from_u32(y1)),
               to_string(undo_xor_shift_right_and(new_from_u32(y2), U, D)));

    let y3 = y2 ^ ((y2 << S) & B);
    assert_eq!(to_string(new_from_u32(y2)),
               to_string(undo_xor_shift_left_and(new_from_u32(y3), S, B)));

    let y4 = y3 ^ ((y3 << T) & C);
    assert_eq!(to_string(new_from_u32(y3)),
               to_string(undo_xor_shift_left_and(new_from_u32(y4), T, C)));

    let y5 = y4 ^ (y4 >> L);
    assert_eq!(to_string(new_from_u32(y4)),
               to_string(undo_xor_shift_right_and(new_from_u32(y5), L, 0xFFFFFFFF)));

    let mut cloned = MtRand {
        state: [0; N],
        index: 0,
    };

    // read N values from mt
    println!("Using {} values to clone RNG", N);
    for _ in 0..N {
        let y5 = new_from_u32(mt.next());
        let y4 = undo_xor_shift_right_and(y5, L, 0xFFFFFFFF);
        let y3 = undo_xor_shift_left_and(y4, T, C);
        let y2 = undo_xor_shift_left_and(y3, S, B);
        let y1 = undo_xor_shift_right_and(y2, U, D);
        cloned.state[cloned.index] = to_u32(y1);
        cloned.index += 1;
    }

    // check that both RNGs are in sync
    println!("Checking both RNGs are in sync");
    let mut ok = true;
    for i in 0..(2 * N) {
        let val1 = mt.next();
        let val2 = cloned.next();
        if val1 != val2 {
            println!("failed at {} ({} != {})", i, val1, val2);
            ok = false;
            break;
        }
    }
    if ok {
        println!("ok");
    }
}

#[derive(Copy, Clone, Debug)]
enum UnAndable {
    Zero,
    One,
    Unknown,
}

fn xor(val1: UnAndable, val2: UnAndable) -> UnAndable {
    return match (val1, val2) {
        (UnAndable::Unknown, _) => UnAndable::Unknown,
        (_, UnAndable::Unknown) => UnAndable::Unknown,
        (UnAndable::Zero, UnAndable::One) => UnAndable::One,
        (UnAndable::One, UnAndable::Zero) => UnAndable::One,
        (UnAndable::Zero, UnAndable::Zero) => UnAndable::Zero,
        (UnAndable::One, UnAndable::One) => UnAndable::Zero,
    };
}

fn new_from_u32(val: u32) -> [UnAndable; 32] {
    let mut r = [UnAndable::Unknown; 32];
    for i in 0..32 {
        let b = (val >> (31 - i)) & 1;
        if b == 0 {
            r[i] = UnAndable::Zero;
        } else {
            r[i] = UnAndable::One;
        }
    }
    return r;
}

fn to_u32(arr: [UnAndable; 32]) -> u32 {
    let mut r = 0;
    for i in 0..32 {
        match arr[i] {
            UnAndable::One => r = r | (1 << (31 - i)),
            UnAndable::Zero => {}
            UnAndable::Unknown => assert!(false),
        }
    }
    return r;
}

fn to_string(arr: [UnAndable; 32]) -> String {
    let mut r = String::new();
    for i in 0..32 {
        if i % 8 == 0 {
            r.push(' ');
        }
        match arr[i] {
            UnAndable::Zero => r.push('0'),
            UnAndable::One => r.push('1'),
            UnAndable::Unknown => r.push('?'),
        }
    }
    return r;
}

/**
 *                             1          2          3
 *                   12345678 90123456 78901234 56789012
 *              y1 = abcdefgh ijklmnop qrstuvwx yzABCDEF
 *         y1 >> U = 00000000 000abcde fghijklm nopqrstu
 *               D = 11111111 11111111 11111111 11111111
 *   (y1 >> U) & D = 00000000 000abcde fghijklm nopqrstu
 *              y2 = abcdefgh ijk..... ........ ........
 */
fn undo_xor_shift_right_and(arr: [UnAndable; 32], shl: u32, and: u32) -> [UnAndable; 32] {
    // TODO: does it matter if amount is less than or greater than 16?
    let mut r = [UnAndable::Unknown; 32];
    for i in 0..32 {
        if ((and >> (31 - i)) & 1) == 0 {
            r[i] = arr[i];
        } else {
            r[i] = UnAndable::Unknown;
        }
    }
    for i in 0..32 {
        if ((and >> (31 - i)) & 1) == 1 {
            if i < shl {
                r[i as usize] = arr[i as usize]
            } else {
                r[i as usize] = xor(arr[i as usize], r[(i - shl) as usize]);
            }
        }
    }
    return r;
}

/**
 *                             1          2          3
 *                   12345678 90123456 78901234 56789012
 *              y2 = abcdefgh ijklmnop qrstuvwx yzABCDEF
 *         y2 << S = hijklmno pqrstuvw xyzABCDE F0000000
 *               B = 10011101 00101100 01010110 10000000
 *   (y2 << S) & B = h00klm0o 00r0tu00 0y0A0CD0 F0000000
 *              y3 = .bc...g. ij.l..op q.s.u..x .zABCDEF
 *
 *
 *                             1          2          3
 *                   12345678 90123456 78901234 56789012
 *              y3 = abcdefgh ijklmnop qrstuvwx yzABCDEF
 *         y3 << T = pqrstuvw xyzABCDE F0000000 00000000
 *               C = 11101111 11000110 00000000 00000000
 *   (y3 << T) & C = pqr0tuvw xy000CD0 00000000 00000000
 *              y4 = ...d.... ..klm..p qrstuvwx yzABCDEF
 */
fn undo_xor_shift_left_and(arr: [UnAndable; 32], shl: u32, and: u32) -> [UnAndable; 32] {
    // TODO: does it matter if amount is less than or greater than 16?
    let mut r = [UnAndable::Unknown; 32];
    for i in 0..32 {
        if ((and >> (31 - i)) & 1) == 0 {
            r[i] = arr[i];
        } else {
            r[i] = UnAndable::Unknown;
        }
    }
    for i in (0..32).rev() {
        // todo: I don't care about i + shl > 31, but we could handle it. It would make this code
        // symmetric with undo_xor_shift_right_and.
        if ((and >> (31 - i)) & 1) == 1 {
            r[i as usize] = xor(arr[i as usize], r[(i + shl) as usize]);
        }
    }
    return r;
}
