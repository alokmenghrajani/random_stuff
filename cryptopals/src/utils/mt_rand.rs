use std::num::Wrapping;

const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;
const A: u32 = 0x9908B0DF;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const F: u32 = 1812433253;
const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = 1 << 31;

pub struct MtRand {
    pub state: [u32; N],
    pub index: usize,
}

impl MtRand {
    pub fn new_with_seed(seed: u32) -> MtRand {
        let mut mt = MtRand {
            state: [0; N],
            index: N,
        };
        mt.state[0] = seed;
        for i in 1..N {
            let a = mt.state[i - 1];
            let d = Wrapping(F) * Wrapping(a ^ (a >> (W - 2))) + Wrapping(i as u32);
            mt.state[i] = d.0;
        }
        return mt;
    }

    pub fn next(&mut self) -> u32 {
        if self.index == N {
            self.twist();
        }

        let y1 = self.state[self.index];
        let y2 = y1 ^ ((y1 >> U) & D);
        let y3 = y2 ^ ((y2 << S) & B);
        let y4 = y3 ^ ((y3 << T) & C);
        let y5 = y4 ^ (y4 >> L);

        self.index += 1;
        return y5;
    }

    #[allow(non_snake_case)]
    fn twist(&mut self) {
        for i in 0..N {
            let x = (Wrapping(self.state[i] & UPPER_MASK) + Wrapping(self.get(i + 1) & LOWER_MASK))
                .0;
            let mut xA = x >> 1;
            if (x % 2) != 0 {
                xA = xA ^ A;
            }
            self.state[i] = self.get(i + M) ^ xA;
        }
        self.index = 0;
    }

    fn get(&self, idx: usize) -> u32 {
        let offset = idx % N;
        return self.state[offset];
    }
}
