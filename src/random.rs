// Based on https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/11
//
// Improvement potential: `thread_rng()`-like thread-local `Rand` object.

use std::time::{SystemTime, UNIX_EPOCH};


const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

pub struct Rand {
    x: u32, y: u32, z: u32, w: u32
}

impl Rand {
    pub fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        Self::new_with_seed(seed)
    }

    pub fn new_with_seed(seed: u32) -> Self {
        Rand {
            x: KX ^ seed, y: KY ^ seed,
            z: KZ, w: KW
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }

    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.is_empty() {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.rand() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }

    pub fn rand_range_inclusive(&mut self, a: i32, b: i32) -> i32 {
        let m = (b - a + 1) as u32;
        return a + (self.rand() % m) as i32;
    }

    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (<u32>::max_value() as f64)
    }
}
