use std::cmp::Ordering;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;

fn query<R: std::io::BufRead, W: std::io::Write>(
    i: usize, read: &mut Reader<R>, write: &mut W
) -> Ordering {
    emitln!(write, "?", i.to1b());
    write.flush().unwrap();
    match read.word().as_str() {
        "<" => Ordering::Less,
        "=" => Ordering::Equal,
        ">" => Ordering::Greater,
        _ => panic!(),
    }
}

fn sort<R: std::io::BufRead, W: std::io::Write>(
    l: usize, r: usize, pmin: usize, pmax: usize, indices: Vec<usize>,
    x: &mut usize, read: &mut Reader<R>, write: &mut W
) -> Vec<usize> {
    assert_eq!(indices.len(), r - l);
    if indices.len() <= 1 {
        return indices;
    }
    let mut q = |i| query(i, read, write);
    let mut set_x = |v| {
        let v = v + 1;
        while v < *x {
            assert_eq!(q(pmin), Ordering::Less);
            *x -= 1;
        }
        while v > *x {
            assert_eq!(q(pmax), Ordering::Greater);
            *x += 1;
        }
    };
    let m = (l + r) / 2;
    set_x(m);
    let mut lower = Vec::new();
    let mut upper = Vec::new();
    for idx in indices {
        match q(idx) {
            Ordering::Less => {
                lower.push(idx);
                q(pmax);
            }
            Ordering::Equal => {
                upper.push(idx);
            }
            Ordering::Greater => {
                upper.push(idx);
                q(pmin);
            }
        }
    }
    let mut lower = sort(l, m, pmin, pmax, lower, x, read, write);
    let mut upper = sort(m, r, pmin, pmax, upper, x, read, write);
    lower.append(&mut upper);
    lower
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();

    let mut q = |i| query(i, read, write);

    let mut pmax = None;
    for i in 0..n {
        match q(i) {
            Ordering::Greater => {
                pmax = Some(i);
                while q(i) == Ordering::Greater {}
            }
            Ordering::Equal => {
                assert!(pmax.is_none());
                pmax = Some(i);
            }
            Ordering::Less => {
                if let Some(pmin) = pmax {
                    assert!(q(pmin) == Ordering::Greater);
                }
            }
        }
    }

    let mut pmin = None;
    for i in 0..n {
        match q(i) {
            Ordering::Less => {
                pmin = Some(i);
                while q(i) == Ordering::Less {}
            }
            Ordering::Equal => {
                assert!(pmin.is_none());
                pmin = Some(i);
            }
            Ordering::Greater => {
                if let Some(pmin) = pmin {
                    assert!(q(pmin) == Ordering::Less);
                }
            }
        }
    }

    let mut x = 1;
    let perm = sort(0, n, pmin.unwrap(), pmax.unwrap(), (0..n).collect(), &mut x, read, write);
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[perm[i]] = i;
    }
    emitln!(write, "!", ans.to1b());
    write.flush().unwrap();
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
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
