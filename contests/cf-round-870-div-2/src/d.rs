use std::cmp;

use contest_lib_rs::{io, emitln};

fn max_pos_in_suffix(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    let mut max_pos_in_suffix = vec![0; n];
    for i in (0..n).rev() {
        max_pos_in_suffix[i] = if i == n - 1 {
            i
        } else {
            if a[i] > a[max_pos_in_suffix[i + 1]] {
                i
            } else {
                max_pos_in_suffix[i + 1]
            }
        };
    }
    max_pos_in_suffix
}

fn max_pos_in_prefix(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    let mut max_pos_in_prefix = vec![0; n];
    for i in 0..n {
        max_pos_in_prefix[i] = if i == 0 {
            i
        } else {
            if a[i] > a[max_pos_in_prefix[i - 1]] {
                i
            } else {
                max_pos_in_prefix[i - 1]
            }
        };
    }
    max_pos_in_prefix
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let b = read.vec_i64(n);

    let c = b.iter().enumerate().map(|(i, &x)| (x - (i as i64))).collect::<Vec<_>>();
    let max_pos_in_c_suffix = max_pos_in_suffix(&c);

    let d = b.iter().enumerate().map(|(i, &x)| (x + (i as i64))).collect::<Vec<_>>();
    let max_pos_in_d_prefix = max_pos_in_prefix(&d);

    let mut answer = i64::MIN;
    for m in 1..(n - 1) {
        let l = max_pos_in_d_prefix[m - 1];
        let r = max_pos_in_c_suffix[m + 1];
        let s = b[l] + b[m] + b[r] - (r - l) as i64;
        answer = cmp::max(answer, s);
    }

    // let mut max_tree = segment_tree::SegmentTree::new(&b, 0, |&x, &y, _| cmp::max(x, y));

    // let mut answer = i64::MIN;
    // for l in 0..(n - 2) {
    //     let m = cmp::min(n - 2, max_pos_in_d_suffix[l + 1]);
    //     {
    //         let b_between = max_tree.get((l + 1) as u32 .. m as u32);
    //         let s = b[l] + b_between + b[m] - (m - l) as i64;
    //         answer = cmp::max(answer, s);
    //     }
    //     {
    //         let r = max_pos_in_c_suffix[m + 1];
    //         let s = b[l] + b[m] + b[r] - (r - l) as i64;
    //         answer = cmp::max(answer, s);
    //     }
    // }
    emitln!(write, answer);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
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
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        5
        5 1 4 2 3
        4
        1 1 1 1
        6
        9 8 7 6 5 4
        7
        100000000 1 100000000 1 100000000 1 100000000
        "), "\
        8
        1
        22
        299999996");
        assert_trimmed_eq!(&run_solver(solve_case, "3   1 1 10"), "10");
        assert_trimmed_eq!(&run_solver(solve_case, "7   1 10 1 100 1 1000 1"), "1106");
    }
}
