// Provides the following functions:
//   - `factorial` (precomputed)
//   - `num_combinations` (additive formula + memoization)
//   - `num_permutations` (multiplicative formula)
//
// All functions are guaranteed to avoid integer overflows assuming the result itself fits in the
// return type.

use std::cell::RefCell;
use std::collections::HashMap;


thread_local! {
    static COMBINATIONS_MEMO: RefCell<HashMap<(i32, i32), i64>> = RefCell::new(HashMap::new());
}

pub fn factorial(n: i32) -> i64 {
    static FACTORIALS: &[i64] = &[
        1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
        6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000,
        6402373705728000, 121645100408832000, 2432902008176640000
    ];
    FACTORIALS[n as usize]
}

pub fn num_combinations(n: i32, k: i32) -> i64 {
    COMBINATIONS_MEMO.with(|memo_storage| {
        let mut memo_borrow = memo_storage.borrow_mut();
        num_combinations_impl(n, k, &mut memo_borrow)
    })
}

fn num_combinations_impl(n: i32, mut k: i32, memo: &mut HashMap<(i32, i32), i64>) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    if n == 0 {
        // Given the condition above, k must be 0 too.
        return 1;
    }
    if k > n - k {
        k = n - k;
    }
    if k == 1 {
        return n as i64;
    }
    if let Some(ret) = memo.get(&(n, k)) {
        return *ret;
    }
    // Use additive formula to avoid overflow.
    let ret = num_combinations_impl(n - 1, k - 1, memo) + num_combinations_impl(n - 1, k, memo);
    memo.insert((n, k), ret);
    ret
}

pub fn num_permutations(n: i32, k: i32) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    let n = n as i64;
    let k = k as i64;
    (n - k + 1..=n).product()
}


#[cfg(test)]
mod tests {
    use crate::assert_panics;
    use super::*;

    #[test]
    fn factorial_raw() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn factorial_all() {
        let mut product: i64 = 1;
        let mut n: i32 = 1;
        while let Some(new_product) = product.checked_mul(n as i64) {
            product = new_product;
            assert_eq!(factorial(n), product);
            n += 1;
        }
        assert_panics!(|| factorial(n));
    }
}
