use std::collections::BTreeSet;

use contest_lib_rs::btreeset_util::OrderedSetNeighborValues;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let a = read.vec_i64(n);
    if k >= 3 {
        emitln!(write, 0);
        return;
    }
    let mut ans1 = *a.iter().min().unwrap();
    let mut ans2 = ans1;
    let a_set = BTreeSet::from_iter(a.iter().copied());
    for i in 0..n {
        for j in i + 1..n {
            let d = (a[i] - a[j]).abs();
            ans1.relax_min(d);
            ans2.relax_min(d);
            if a_set.contains(&d) {
                ans2 = 0;
            } else {
                if let Some(x) = a_set.prev_value(&d) {
                    ans2.relax_min((d - x).abs());
                }
                if let Some(x) = a_set.next_value(&d) {
                    ans2.relax_min((d - x).abs());
                }
            }
        }
    }
    match k {
        1 => emitln!(write, ans1),
        2 => emitln!(write, ans2),
        _ => unreachable!(),
    }
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
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
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
        assert_trimmed_eq!(&run_solver(solve_case, "3 2  10 100 112"), "2");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        5 2
        3 9 7 15 1
        4 3
        7 4 15 12
        6 2
        42 47 50 54 62 79
        2 1
        500000000000000000 1000000000000000000
        "), "\
        1
        0
        3
        500000000000000000");
    }
}
