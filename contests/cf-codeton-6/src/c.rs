use std::collections::BTreeSet;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let a = read.vec_usize(n).from1b();

    let colors = a.iter().copied().collect::<BTreeSet<_>>();
    let mut color_pos = vec![BTreeSet::new(); k];
    for i in 0..n {
        color_pos[a[i]].insert(i);
    }

    let mut ans = vec![0; k];
    let mut agg_pos = BTreeSet::new();
    for c in (0..k).rev() {
        if !colors.contains(&c) {
            continue;
        }
        agg_pos.extend(color_pos[c].iter().copied());
        let min = agg_pos.first().unwrap();
        let max = agg_pos.last().unwrap();
        ans[c] = (max - min + 1) * 2;
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
        assert_trimmed_eq!(&run_solver(solve_case, "2 2  1 2"), "4 2");
        assert_trimmed_eq!(&run_solver(solve_case, "3 3  2 1 3"), "6 6 2");
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        2 1
        1 1
        2 2
        1 2
        3 5
        3 2 4
        4 2
        1 2 1 2
        5 3
        1 2 3 2 1
        "), "\
        4
        4 2
        0 6 6 2 0
        8 6
        10 6 2");
    }
}
