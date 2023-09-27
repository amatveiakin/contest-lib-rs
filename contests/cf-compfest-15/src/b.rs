use contest_lib_rs::factored_num::FactoredNum;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::num::IntegerRing;

type M = ModNumber<998244353>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let b = read.vec_u32(n);
    let m = read.usize();
    let c = read.vec_u32(m);
    let d = read.vec_u32(m);
    let x = FactoredNum::from_factors(a.into_iter().zip(b.into_iter()));
    let y = FactoredNum::from_factors(c.into_iter().zip(d.into_iter()));
    let ans =
        if let Some(r) = x / y {
            M::from(2).pow(r.factors().len() as u32)
        } else {
            M::zero()
        };
    emitln!(write, ans);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        2 3 5 7
        2 1 1 2
        2
        3 7
        1 1"), "8");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        1299721 1999993
        100000 265
        2
        1299721 1999993
        100000 265"), "1");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        2 5
        1 1
        2
        2 3
        1 1"), "0");
    }
}
