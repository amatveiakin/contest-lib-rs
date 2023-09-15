use std::collections::HashMap;

use contest_lib_rs::iterutils_dedup::IterutilsDedup;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let b = read.vec_i32(n);
    let mut a_count = HashMap::new();
    let mut b_count = HashMap::new();
    a.into_iter().dedup_with_count().for_each(|(c, v)| {
        let count: &mut usize = a_count.entry(v).or_insert(0);
        *count = (*count).max(c);
    });
    b.into_iter().dedup_with_count().for_each(|(c, v)| {
        let count: &mut usize = b_count.entry(v).or_insert(0);
        *count = (*count).max(c);
    });
    let mut answer = 0;
    for (&v, &c) in a_count.iter() {
        let total = c + *b_count.get(&v).unwrap_or(&0);
        answer = answer.max(total);
    }
    for (&v, &c) in b_count.iter() {
        let total = c + *a_count.get(&v).unwrap_or(&0);
        answer = answer.max(total);
    }
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        1
        2
        2
        3
        1 2 3
        4 5 6
        2
        1 2
        2 1
        5
        1 2 2 2 2
        2 1 1 1 1
        "), "\
        2
        1
        2
        5");
    }
}
