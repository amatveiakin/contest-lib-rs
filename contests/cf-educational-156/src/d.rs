use std::io::BufWriter;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::num::RingNumber;

type ModNum = ModNumber<998244353>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let mut s = read.word_as_chars();
    assert_eq!(s.len(), n - 1);

    let mut prod = ModNum::one();
    for i in 1..n - 1 {
        if s[i] == '?' {
            prod *= ModNum::from(i as u32);
        }
    }

    for iter in 0..=m {
        if iter > 0 {
            let i = read.usize().from1b();
            let [c] = read.word_as_char_array();
            let old_c = s[i];
            if i > 0 {
                let imod = ModNum::from(i as u32);
                if old_c == '?' && c != '?' {
                    prod /= imod;
                } else if old_c != '?' && c == '?' {
                    prod *= imod;
                }
            }
            s[i] = c;
        }
        let zero = s[0] == '?';
        let ans = if zero { ModNum::zero() } else { prod };
        emitln!(write, ans);
    }
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use std::fmt::Write;

    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        6 4
        <?>?>
        1 ?
        4 <
        5 <
        1 >
        "), "\
        3
        0
        0
        0
        1");
        assert_trimmed_eq!(&run_solver(solve, "\
        2 2
        >
        1 ?
        1 <
        "), "\
        1
        0
        1");
    }
}
