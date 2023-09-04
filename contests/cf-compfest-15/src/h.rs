#![allow(non_snake_case)]

use contest_lib_rs::combinatorics_mod::factorial_mod;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;

type M = ModNumber<998244353>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.i32s();
    let N = M::from(n);
    let K = M::from(k);
    let mut a = M::from(0);
    let ONE = M::from(1);
    for i in (n - k).max(0)..n {
        let I = M::from(i);
        a += factorial_mod(n) / factorial_mod(i) * (
            factorial_mod(n - i) * (N - I + ONE).pow((k - n + i + 1).try_into().unwrap())
            - factorial_mod(n - (i + 1)) * (N - (I + ONE) + ONE).pow((k - n + (i + 1) + 1).try_into().unwrap())
        )
    }
    a += ONE;
    emitln!(write, a);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "2 2"), "11");
        assert_trimmed_eq!(&run_solver(solve, "1 3"), "8");
        assert_trimmed_eq!(&run_solver(solve, "3 1"), "4");
    }
}
