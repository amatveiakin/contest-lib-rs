// Implements a subset of `rand` crate interface.
// Usage examples:
//   let x = rand::random();
//   my_slice.shuffle(&mut rand::thread_rng());
//
// Distribution of `rand::random` results:
//   - Integers: Uniformly distributed over all values of the type.
//   - Floating point types: Uniformly distributed in the half-open range [0, 1).
//   - Boolean: `true` of `false` with equal probability.
//
// Kudos to https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/11
// for the random number generator.

use std::time::{SystemTime, UNIX_EPOCH};


const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

pub fn random<T>() -> T where Standard: Distribution<T> {
    RAND.with(|r| r.borrow_mut().gen())
}

thread_local! {
    pub static RAND: std::cell::RefCell<Rand> = std::cell::RefCell::new(Rand::new())
}

pub trait Rng {
    fn next_u32(&mut self) -> u32;
    fn gen<T>(&mut self) -> T where Standard: Distribution<T> {
        Standard.sample(self)
    }
}

pub trait SliceRandom {
    fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R);
}

pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

pub struct Standard;

pub struct ThreadRng;

pub fn thread_rng() -> ThreadRng {
    ThreadRng
}

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
}

impl Rng for Rand {
    // Xorshift 128, taken from German Wikipedia
    fn next_u32(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }
}

impl Rng for ThreadRng {
    fn next_u32(&mut self) -> u32 {
        RAND.with(|r| r.borrow_mut().next_u32())
    }
}

impl Distribution<u32> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
        rng.next_u32()
    }
}
impl Distribution<i32> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i32 {
        rng.next_u32() as i32
    }
}
impl Distribution<u64> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        (rng.next_u32() as u64) << 32 | (rng.next_u32() as u64)
    }
}
impl Distribution<i64> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i64 {
        (rng.next_u32() as i64) << 32 | (rng.next_u32() as i64)
    }
}

impl Distribution<f32> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        (rng.next_u32() as f32) / (<u32>::max_value() as f32)
    }
}
impl Distribution<f64> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        (rng.next_u32() as f64) / (<u32>::max_value() as f64)
    }
}

impl Distribution<bool> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.next_u32() % 2 == 0
    }
}

impl<T> SliceRandom for [T] {
    fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        assert!(self.len() <= u32::MAX as usize);
        if self.is_empty() {
            return;
        }
        let mut i = self.len() - 1;
        while i > 0 {
            let j = (rng.next_u32() as usize) % (i + 1);
            self.swap(i, j);
            i -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i64_distribution() {
        const N: u32 = 10_000;
        // Should work with any seed, but picking one for stability.
        let mut rng = Rand::new_with_seed(42);

        let mut b1 = 0;
        let mut b2 = 0;
        let mut b3 = 0;
        let mut b4 = 0;
        for _ in 0..N {
            let x = rng.gen::<i64>();
            if x < i64::MIN / 2 {
                b1 += 1;
            } else if x < 0 {
                b2 += 1;
            } else if x < i64::MAX / 2 {
                b3 += 1;
            } else {
                b4 += 1;
            }
        }

        let check_bucket = |b| {
            assert!(b > N / 5);
            assert!(b < N / 3);
        };
        check_bucket(b1);
        check_bucket(b2);
        check_bucket(b3);
        check_bucket(b4);
    }
}
