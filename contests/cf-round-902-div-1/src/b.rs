use contest_lib_rs::divisors::divisors;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::mod_ring::ModNumber;

type ModNum = ModNumber::<998244353>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let mut a = a.into_iter().enumerate().collect_vec();
    a.sort_by_key(|(_, x)| std::cmp::Reverse(*x));
    let mut t = ModNum::from(0);
    let mut mask = vec![true; n];
    let mut free = n;
    for (i, v) in a {
        let mut new = 0;
        for j in divisors(i + 1) {
            if mask[j - 1] {
                new += 1;
                mask[j - 1] = false;
            }
        }
        t += ModNum::from(v) *
            (ModNum::from(2).pow(free as u32) - ModNum::from(2).pow((free - new) as u32));
        free -= new;
    }
    emitln!(write, t);
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
        assert_trimmed_eq!(&run_solver(solve, "1  0"), "0");
        assert_trimmed_eq!(&run_solver(solve, "1  10"), "10");
        assert_trimmed_eq!(&run_solver(solve, "2  1 5"), "15");
        assert_trimmed_eq!(&run_solver(solve, "4  19 14 19 9"), "265");
        assert_trimmed_eq!(&run_solver(solve, "15  90000 9000 99000 900 90900 9900 99900 90 90090 9090 99090 990 90990 9990 99990"), "266012571");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
