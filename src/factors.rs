use crate::primes::primes;

// Given `x` returns [(p_1, k_1), ..., (p_n, k_n)] such that:
//   - x = p_1^k_1 * ... * p_n^k_n,
//   - p_i is prime,
//   - p_i < p_{i+1},
pub fn factors(mut x: u32) -> Vec<(u32, u32)> {
    assert!(x > 0);
    let mut ret = vec![];
    for p in primes() {
        if x == 1 {
            return ret;
        }
        if p * p > x {
            ret.push((x, 1));
            return ret;
        }
        if x % p == 0 {
            let mut k = 0;
            while x % p == 0 {
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
    use crate::internal_testing::catch_unwind_silent;

    use super::*;

    #[test]
    fn basic() {
        assert!(catch_unwind_silent(|| factors(0)).is_err());
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
}
