use std::collections::HashSet;

use contest_lib_rs::{io, emitln};
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);

    let mut z = HashSet::new();
    let mut s1 = vec![];
    let mut s2 = vec![];

    let mut p = None;
    let mut m = 0;
    for i in 0..(n + 1) {
        let v = *a.get(i).unwrap_or(&0);
        if v == 0 {
            if let Some(pp) = p {
                if m == 1 {
                    s1.push((pp as i32, i as i32 - 1));
                } else {
                    s2.push((pp as i32, i as i32 - 1));
                }
                p = None;
                m = 0;
            }
            if i < n {
                z.insert(i as i32);
            }
        } else {
            if p.is_none() {
                p = Some(i);
            }
            m.relax_max(v);
        }
    }

    for &(l, r) in &s2 {
        z.remove(&(l - 1));
        z.remove(&(r + 1));
    }
    for &(l, r) in &s1 {
        if !z.remove(&(l - 1)) {
            z.remove(&(r + 1));
        }
    }

    let total = s2.len() + s1.len() + z.len();
    emitln!(write, total);
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
        assert_trimmed_eq!(&run_solver(solve, "3  0 2 0"), "1");
        assert_trimmed_eq!(&run_solver(solve, "4  0 0 1 1"), "2");
        assert_trimmed_eq!(&run_solver(solve, "7  0 1 0 0 1 0 2"), "4");
        assert_trimmed_eq!(&run_solver(solve, "6  0 1 0 1 0 2"), "3");
        assert_trimmed_eq!(&run_solver(solve, "6  2 0 1 0 1 0"), "3");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
