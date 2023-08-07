// Provides the following functions:
//   - `factorial` / `factorial_mod`
//   - `num_combinations` / `num_combinations_mod`
//   - `num_permutations` / `num_permutations_mod`
//
// All functions are guaranteed to avoid integer overflows assuming the result itself fits in the
// return type.
//
// Most functions (namely, all expect `num_permutations`) are memoized or precomputed, and thus
// executed in amortized O(1) time.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::mod_ring::ModNumber;


// Improvement potential: Generic function with generic memoization when Rust supports it.
//   Something like:
//
// pub fn factorial<T>(n: i32) -> T
// where
//     T: IntegerRing,
//     i32: Into<T>,
// {
//     thread_local! {
//         static FACTORIAL_MEMO: FactorialMemo<T> = FactorialMemo { values: vec![T::one()] };
//     }
//     ...
// }

thread_local! {
    static COMBINATIONS_MEMO: RefCell<HashMap<(i32, i32), i64>> = RefCell::new(HashMap::new());
    static FACTORIAL_MOD_MEMO: RefCell<HashMap<i32, Vec<i32>>> = RefCell::new(HashMap::new());
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

pub fn factorial_mod<const M: i32>(n: i32) -> ModNumber<M> {
    FACTORIAL_MOD_MEMO.with(|memo_storage| {
        let mut memo_borrow = memo_storage.borrow_mut();
        let memo = memo_borrow.entry(M).or_insert_with(|| vec![1.into()]);
        while memo.len() <= n as usize {
            let last = ModNumber::<M>::from(*memo.last().unwrap());
            let next = last * (memo.len() as i32).into();
            memo.push(next.val());
        }
        ModNumber::from(memo[n as usize])
    })
}

pub fn num_combinations_mod<const M: i32>(n: i32, k: i32) -> ModNumber<M> {
    if k < 0 || k > n {
        return ModNumber::from(0);
    }
    factorial_mod(n) / (factorial_mod(k) * factorial_mod(n - k))
}

pub fn num_permutations_mod<const M: i32>(n: i32, k: i32) -> ModNumber<M> {
    if k < 0 || k > n {
        return ModNumber::from(0);
    }
    factorial_mod(n) / factorial_mod(n - k)
}


#[cfg(test)]
mod tests {
    use crate::assert_panics;
    use crate::mod_ring::ModNumber;
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
    fn factorial_mod_ring() {
        type Mod5 = ModNumber<5>;
        assert_eq!(factorial_mod(0), Mod5::from(1));
        assert_eq!(factorial_mod(1), Mod5::from(1));
        assert_eq!(factorial_mod(2), Mod5::from(2));
        assert_eq!(factorial_mod(3), Mod5::from(1));
        assert_eq!(factorial_mod(4), Mod5::from(4));
        assert_eq!(factorial_mod(5), Mod5::from(0));

        type Mod101 = ModNumber<101>;
        assert_eq!(factorial_mod(0), Mod101::from(1));
        assert_eq!(factorial_mod(1), Mod101::from(1));
        assert_eq!(factorial_mod(2), Mod101::from(2));
        assert_eq!(factorial_mod(3), Mod101::from(6));
        assert_eq!(factorial_mod(4), Mod101::from(24));
        assert_eq!(factorial_mod(5), Mod101::from(19));
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

    #[test]
    fn compare_combinations() {
        type ModNum = ModNumber<1_000_000_007>;
        for n in 0..=10 {
            for k in 0..=10 {
                let c_regular = num_combinations(n, k);
                let c_mod: ModNum = num_combinations_mod(n, k);
                assert_eq!(i32::try_from(c_regular).unwrap(), c_mod.val(), "n={}, k={}", n, k);
            }
        }
    }

    #[test]
    fn compare_permutations() {
        type ModNum = ModNumber<1_000_000_007>;
        for n in 0..=10 {
            for k in 0..=10 {
                let p_regular = num_permutations(n, k);
                let p_mod: ModNum = num_permutations_mod(n, k);
                assert_eq!(i32::try_from(p_regular).unwrap(), p_mod.val(), "n={}, k={}", n, k);
            }
        }
    }
}
