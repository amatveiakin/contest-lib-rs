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

fn fill_divisors<T: RegularInteger>(
    i: usize, mut v: T, factorization: &[(T, u32)], ret: &mut Vec<T>
) {
    if i == factorization.len() {
        ret.push(v);
    } else {
        let (p, k) = factorization[i];
        for _ in 0..=k {
            fill_divisors(i + 1, v, factorization, ret);
            v *= p;
        }
    }
}

// Returns the list of all positive integers that divide `x`, including 1 and `x` itself.
// The list is NOT sorted.
pub fn divisors<T: RegularInteger>(x: T) -> Vec<T> {
    let factorization = factors(x);
    let mut ret = vec![];
    fill_divisors(0, T::one(), &factorization, &mut ret);
    ret
}


#[cfg(test)]
mod tests {
    use crate::assert_panics;
    use super::*;

    #[test]
    fn factorize_basic() {
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
    fn factorize_big_numbers() {
        let non_u32_prime = 4_294_967_311_u64;
        assert_eq!(factors(non_u32_prime), vec![(non_u32_prime, 1)]);
        assert_eq!(factors(5_000_000_000_u64), vec![(2, 9), (5, 10)]);
        assert_eq!(factors(u64::MAX),
            vec![(3, 1), (5, 1), (17, 1), (257, 1), (641, 1), (65537, 1), (6700417, 1)]);
        // Would've been nice to test primes close to `u64::MAX`, but it takes forever.
    }

    #[test]
    fn factorize_too_big_numbers() {
        assert_panics!(|| factors(1u128));
    }

    #[track_caller]
    fn check_divisors(x: u32, expected: &[u32]) {
        let mut actual = divisors(x);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn all_factors_basic() {
        check_divisors(1, &[1]);
        check_divisors(2, &[1, 2]);
        check_divisors(3, &[1, 3]);
        check_divisors(4, &[1, 2, 4]);
        check_divisors(64, &[1, 2, 4, 8, 16, 32, 64]);
        check_divisors(6, &[1, 2, 3, 6]);
        check_divisors(12, &[1, 2, 3, 4, 6, 12]);
        check_divisors(60, &[1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
        check_divisors(1001, &[1, 7, 11, 13, 77, 91, 143, 1001]);
    }
}
