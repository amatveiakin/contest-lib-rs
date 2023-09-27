use crate::num::RegularInteger;
use crate::primes::{primes, PrimesIter};

// Given `x` returns [(p_1, k_1), ..., (p_n, k_n)] such that:
//   - x = p_1^k_1 * ... * p_n^k_n,
//   - p_i is prime,
//   - p_i < p_{i+1},
pub fn factors<T: RegularInteger>(mut x: T) -> Vec<(T, u32)> {
    // If size of x type is twice the size of prime type or less, then p * p > x is implied when
    // primes are over. Note that for now factoring numbers close to u64::MAX wouldn't work because
    // primes iterator never ends.
    assert!(std::mem::size_of::<T>() <= 2 * std::mem::size_of::<<PrimesIter as Iterator>::Item>());

    assert!(x > T::zero());
    let mut ret = vec![];
    for p in primes() {
        if x == T::one() {
            return ret;
        }
        let p = T::from_u32(p);
        if p * p > x {
            ret.push((x, 1));
            return ret;
        }
        if x % p == T::zero() {
            let mut k = 0;
            while x % p == T::zero() {
                x /= p;
                k += 1;
            }
            ret.push((p, k));
        }
    }
    unreachable!("primes() should never end");
}


#[cfg(test)]
mod tests {
    use crate::assert_panics;
    use super::*;

    #[test]
    fn basic() {
        assert_panics!(|| factors(0));
        assert_eq!(factors(1), vec![]);
        assert_eq!(factors(2), vec![(2, 1)]);
        assert_eq!(factors(3), vec![(3, 1)]);
        assert_eq!(factors(97), vec![(97, 1)]);
        assert_eq!(factors(4), vec![(2, 2)]);
        assert_eq!(factors(1024), vec![(2, 10)]);
        assert_eq!(factors(6), vec![(2, 1), (3, 1)]);
        assert_eq!(factors(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factors(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(factors(160), vec![(2, 5), (5, 1)]);
        assert_eq!(factors(1001), vec![(7, 1), (11, 1), (13, 1)]);
    }

    #[test]
    fn big_numbers() {
        let non_u32_prime = 4_294_967_311_u64;
        assert_eq!(factors(non_u32_prime), vec![(non_u32_prime, 1)]);
        assert_eq!(factors(5_000_000_000_u64), vec![(2, 9), (5, 10)]);
        assert_eq!(factors(u64::MAX),
            vec![(3, 1), (5, 1), (17, 1), (257, 1), (641, 1), (65537, 1), (6700417, 1)]);
        // Would've been nice to test primes close to `u64::MAX`, but it takes forever.
    }

    #[test]
    fn too_big_numbers() {
        assert_panics!(|| factors(1u128));
    }
}
