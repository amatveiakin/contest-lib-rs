use std::collections::HashSet;

use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let s = read.word().chars().map(|c| if c == '1' { 1 } else { 0 }).collect::<Vec<_>>();

    let mut decr = vec![0; n];
    for i in 1..n {
        decr[i] = decr[i - 1];
        if s[i] == 0 && s[i - 1] == 1 {
            decr[i] += 1;
        }
    }

    let mut prev0 = vec![0; n];
    for i in 0..n {
        if s.get(i.overflowing_sub(1).0) == Some(&0) {
            prev0[i] = prev0[i - 1];
        } else {
            prev0[i] = i;
        }
    }
    let mut next1 = vec![0; n];
    for i in (0..n).rev() {
        if s.get(i + 1) == Some(&1) {
            next1[i] = next1[i + 1];
        } else {
            next1[i] = i;
        }
    }

    let mut lr = HashSet::new();
    for _ in 0..m {
        let [l, r] = read.usizes();
        let l = l - 1;
        let r = r - 1;
        if decr[l] == decr[r] {
            lr.insert((2_000_000_000, 2_000_000_000));
        } else {
            let l = prev0[l];
            let r = next1[r];
            lr.insert((l, r));
        }
    }
    emitln!(write, lr.len());
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
        3
        6 5
        101100
        1 2
        1 3
        2 4
        5 5
        1 6
        6 4
        100111
        2 2
        1 4
        1 3
        1 2
        1 1
        0
        1 1"), "\
        3
        3
        1");
    }
}
