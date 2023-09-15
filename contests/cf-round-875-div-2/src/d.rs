use std::{collections::BTreeMap, cmp::Ordering};

use contest_lib_rs::iterutils_dedup::IterutilsDedup;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let b = read.vec_i32(n);
    let mut ab_raw: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for i in 0..n {
        ab_raw.entry(a[i]).or_default().push(b[i]);
    }
    let mut ab = BTreeMap::new();
    for (&a, bv) in ab_raw.iter_mut() {
        bv.sort();
        ab.insert(a, bv.iter().copied().group_identical().collect::<Vec<_>>());
    }
    let mut answer: i64 = 0;
    let mut i = 0;
    for (&a1, bv1) in ab.iter() {
        i += 1;
        let mut j = 0;
        for (&a2, bv2) in ab.iter() {
            j += 1;
            if j > i {
                break;
            }
            let s = a1 as i64 * a2 as i64;
            if s > 2 * n as i64 {
                break;
            }
            let s = s as i32;
            let mut p1: i32 = 0;
            let mut p2: i32 = bv2.len() as i32 - 1;
            while p1 < bv1.len() as i32 && p2 >= 0 {
                let (b1, bc1) = bv1[p1 as usize];
                let (b2, bc2) = bv2[p2 as usize];
                let bc1 = bc1 as i64;
                let bc2 = bc2 as i64;
                match (b1 + b2).cmp(&s) {
                    Ordering::Less => p1 += 1,
                    Ordering::Equal => {
                        if a1 == a2 && p1 > p2 {
                            break;
                        }
                        if a1 == a2 && b1 == b2 {
                            answer += bc1 * (bc1 - 1) / 2;
                            break;
                        }
                        answer += bc1 * bc2;
                        p1 += 1;
                        p2 -= 1;
                    },
                    Ordering::Greater => p2 -= 1,
                }
            }
        }
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        8
        4 4 4 4 4 4 4 4
        8 8 9 9 9 9 9 9
        8
        4 4 4 4 4 4 4 4
        8 8 7 7 7 7 7 7
        "), "\
        1
        1");
        assert_trimmed_eq!(&run_solver(solve, "\
        1
        8
        3 3 3 3 3 3 3 3
        1 1 3 3 6 6 8 100
        "), "\
        6");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        3
        2 3 2
        3 3 1
        8
        4 2 8 2 1 2 7 5
        3 5 8 8 1 1 6 5
        8
        4 4 8 8 8 8 8 8
        8 8 8 8 8 8 8 8
        "), "\
        2
        7
        1");
    }
}
