// UNFINISHED, wrong concept

use std::collections::HashSet;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::iterutils_is_sorted::IterutilsIsSorted;

fn sort_array(mut a: Vec<i32>) -> Option<Vec<usize>> {
    let mut seen = HashSet::new();
    let mut ops = vec![];
    while !a.iter().issorted() {
        let pivot = a[0] - 1;
        let p = a.iter().position(|&x| x == pivot).unwrap_or(0);
        let b = [&a[(p + 1)..], &[a[p]][..], &a[..p]].concat();
        if !seen.insert(b.clone()) {
            return None;
        }
        // println!("### {a:?} @{p} -> {b:?}");
        a = b;
        ops.push(p);
    }
    Some(ops)
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let p = read.vec_i32(n).from1b();
    let q = read.vec_i32(m).from1b();

    let (Some(mut op), Some(mut oq)) = (sort_array(p), sort_array(q)) else {
        emitln!(write, -1);
        return;
    };

    if op.len().abs_diff(oq.len()) % 2 == 1 {
        if n % 2 == 1 {
            for i in 0..n {
                op.push(0);
            }
        } else if m % 2 == 1 {
            for i in 0..m {
                oq.push(0);
            }
        } else {
            emitln!(write, -1);
            return;
        }
    }
    while op.len() < oq.len() {
        op.push(0);
        op.push(n - 1);
    }
    while oq.len() < op.len() {
        oq.push(0);
        oq.push(m - 1);
    }
    let opq = op.into_iter().zip(oq.into_iter()).collect_vec();

    emitln!(write, opq.len());
    for (ap, aq) in opq {
        let [ap, aq] = [ap, aq].to1b();
        emitln!(write, ap, aq);
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3 5
        // 2 1 3
        // 5 2 1 4 3
        // "), "\
        // ");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4 4
        // 3 4 2 1
        // 2 4 1 3
        // "), "\
        // 5
        // 4 2
        // 3 3
        // 1 4
        // 3 2
        // 4 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        2 2
        1 2
        2 1
        "), "-1");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
