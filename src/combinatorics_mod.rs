// Provides the following functions:
//   - `factorial_mod`
//   - `num_combinations_mod`
//   - `num_permutations_mod`
//
// All functions are based on memoized factorial implementation and thus are executed in O(1)
// amortized time.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::mod_ring::ModNumber;


// Improvement potential: Memoization based on type, not mod value, when Rust supports it.
//   Something like:
//
// pub fn factorial_mod<const M: i32>(n: i32) -> ModNumber<M> {
//     thread_local! {
//         static FACTORIAL_MEMO: FactorialMemo = FactorialMemo { ... };
//     }
//     ...
// }

thread_local! {
    static FACTORIAL_MOD_MEMO: RefCell<HashMap<i32, Vec<i32>>> = RefCell::new(HashMap::new());
}

pub fn factorial_mod<const M: i32>(n: i32) -> ModNumber<M> {
    FACTORIAL_MOD_MEMO.with_borrow_mut(|memo_map| {
        let memo = memo_map.entry(M).or_insert_with(|| vec![1.into()]);
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
    use crate::combinatorics::{num_combinations, num_permutations};
    use crate::mod_ring::ModNumber;
    use super::*;

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
