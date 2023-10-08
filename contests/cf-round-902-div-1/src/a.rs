use std::collections::BTreeMap;

use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let p = read.u32();
    let va = read.vec_usize(n);
    let vb = read.vec_u32(n);

    let mut left = BTreeMap::new();
    for (a, b) in va.into_iter().zip(vb.into_iter()) {
        left.entry(b).or_insert(vec![]).push(a);
    }

    let mut ann = CountingSet::new();
    ann.push_multiple(p, n);

    let mut cost: i64 = 0;
    while let Some((b, mut aa)) = left.pop_first() {
        let a = aa.pop().unwrap();
        cost += ann.pop_first().unwrap() as i64;
        ann.push_multiple(b, a);
        if !aa.is_empty() {
            left.insert(b, aa);
        }
    }
    emitln!(write, cost);
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
        3
        6 3
        2 3 2 1 1 3
        4 3 2 6 3 6
        1 100000
        100000
        1
        4 94
        1 4 2 3
        103 96 86 57
        "), "\
        16
        100000
        265");
    }
}
