use crate::factors::factors;
use crate::num::Integer;


// Returns the list of all positive integers that divide `x`, including 1 and `x` itself.
// The list is NOT sorted.
pub fn divisors<T: Integer>(x: T) -> Vec<T> {
    let factorization = factors(x);
    let mut ret = vec![];
    fill_divisors(0, T::one(), &factorization, &mut ret);
    ret
}

fn fill_divisors<T: Integer>(
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


#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check_divisors(x: u32, expected: &[u32]) {
        let mut actual = divisors(x);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn basic() {
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
