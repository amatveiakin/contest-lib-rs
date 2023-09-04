use contest_lib_rs::{io, emitln};

use std::cmp;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.i32();
    let m = read.i32();
    let x = read.vec_i32(n as usize);
    let num_s1 = x.iter().filter(|&&v| v == -1).count() as i32;
    let num_s2 = x.iter().filter(|&&v| v == -2).count() as i32;
    let mut s3: Vec<_> = x.iter().filter(|&&v| v > 0).collect();
    s3.sort();
    s3.dedup();
    let num_s3 = s3.len() as i32;
    let mut best_answer = 0;

    {
        let answer = cmp::min(m, num_s2 + num_s3);
        best_answer = cmp::max(best_answer, answer);
    }

    {
        let answer = cmp::min(m, num_s1 + num_s3);
        best_answer = cmp::max(best_answer, answer);
    }

    for (idx, &&pos) in s3.iter().enumerate() {
        let idx = idx as i32;
        let seats_left = pos - 1;
        let seats_right = m - pos;
        let s3_people_left = idx;
        let s3_people_right = num_s3 - idx - 1;
        let people_left = cmp::min(seats_left, num_s1 + s3_people_left);
        let people_right = cmp::min(seats_right, num_s2 + s3_people_right);
        let answer = 1 + people_left + people_right;
        best_answer = cmp::max(best_answer, answer);
    }

    emitln!(write, best_answer);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        10
        3 10
        5 5 5
        4 6
        1 -2 -2 1
        5 7
        -1 -1 4 -2 -2
        6 7
        5 -2 -2 -2 -2 -2
        6 6
        -1 1 4 5 -1 4
        6 8
        -1 -1 -1 3 -1 -2
        6 7
        5 -1 -2 -2 -2 -2
        3 1
        -2 -2 1
        2 5
        5 -2
        1 2
        -1
        "), "\
        1
        3
        5
        6
        5
        5
        5
        1
        2
        1");
    }
}
