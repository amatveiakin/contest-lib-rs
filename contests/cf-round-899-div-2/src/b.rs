use std::collections::HashSet;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::bitset::Bitset;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    const MAX: usize = 50;
    let n = read.usize();
    let mut sets = HashSet::new();
    for _ in 0..n {
        let mut b = Bitset::new(MAX);
        let k = read.usize();
        for _ in 0..k {
            let s = read.usize().from1b();
            b.set(s, true);
        }
        sets.insert(b);
    }
    let sets = sets.into_iter().collect_vec();
    let total_union = sets.iter().fold(Bitset::new(MAX), |acc, x| acc | x);
    let mut ans = 0;
    for p in 0..MAX {
        let mut union = Bitset::new(MAX);
        for b in &sets {
            if !b.get(p) {
                union |= b;
            }
        }
        if union != total_union {
            ans.relax_max(union.count());
        }
    }
    emitln!(write, ans);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
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
        3
        3 1 2 3
        2 4 5
        2 3 4
        4
        4 1 2 3 4
        3 2 5 6
        3 3 5 6
        3 4 5 6
        5
        1 1
        3 3 6 10
        1 9
        2 1 3
        3 5 8 9
        1
        2 4 28
        "), "\
        4
        5
        6
        0");
    }
}
