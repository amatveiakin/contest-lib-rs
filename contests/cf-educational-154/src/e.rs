// UNFINISHED

use contest_lib_rs::combinatorics_mod::factorial_mod;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;

type M = ModNumber<998244353>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.i32s();
    let kmod = M::from(k);
    let mut a = vec![M::from(0); k as usize];
    for i in k..=n {
        let v = factorial_mod(k) * (kmod.pow((i - k) as u32) + a[(i - k) as usize])
            + a[(i - 1) as usize];
        a.push(v);
    }
    emitln!(write, a.last().unwrap());
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "2 2"), "2");
        assert_trimmed_eq!(&run_solver(solve, "3 2"), "6");
        assert_trimmed_eq!(&run_solver(solve, "4 2"), "18");
        assert_trimmed_eq!(&run_solver(solve, "5 2"), "46");
        // assert_trimmed_eq!(&run_solver(solve, "10 3"), "71712");
        // assert_trimmed_eq!(&run_solver(solve, "1337 42"), "524933698");
    }
}
