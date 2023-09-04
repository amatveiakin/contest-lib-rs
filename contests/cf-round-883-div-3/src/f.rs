use std::io::Write;

use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let mut n = read.usize();
    let initial_a_set: CountingSet<_> = read.vec_i32(n).into_iter().collect();
    let mut special_t = None;
    emitln!(write, "- 0");
    std::io::stdout().flush().unwrap();
    loop {
        let a = read.vec_i32(n);
        if let Some(special_t) = special_t {
            let p = a.iter().position(|&x| x != special_t);
            if let Some(p) = p {
                emitln!(write, "!", p + 1);
                std::io::stdout().flush().unwrap();
                return;
            } else {
                emitln!(write, "- 0");
            }
        } else {
            let a_set: CountingSet<_> = a.iter().copied().collect();
            if a_set != initial_a_set {
                let mut t = None;
                for (&v, count) in a_set.iter_groups() {
                    if count > initial_a_set.count(&v) {
                        t = Some(v);
                        break;
                    }
                }
                let t = t.unwrap();
                let to_remove = a.iter().enumerate()
                    .filter(|(_, &x)| x != t)
                    .map(|(i, _)| i + 1)
                    .collect::<Vec<_>>();
                special_t = Some(t);
                n -= to_remove.len();
                emitln!(write, "-", to_remove.len(), to_remove);
            } else {
                emitln!(write, "- 0");
            }
        }
        std::io::stdout().flush().unwrap();
    }
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
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
