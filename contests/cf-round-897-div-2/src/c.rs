use std::collections::HashSet;
use std::process::exit;

use contest_lib_rs::io::prelude::*;

fn get_mex(a: &HashSet<i32>) -> i32 {
    let mut a = a.iter().copied().collect::<Vec<_>>();
    a.sort();
    let n = a.len() as i32;
    for i in 0..n {
        if a[i as usize] != i {
            return i;
        }
    }
    n
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i32();
    let mut s = read.vec_i32(n as usize).into_iter().collect::<HashSet<_>>();
    let mex = get_mex(&s);
    s.insert(mex);
    emitln!(write, mex);
    write.flush().unwrap();
    loop {
        let bob = read.i32();
        match bob {
            -1 => return,
            -2 => exit(0),
            v => {
                emitln!(write, v);
                write.flush().unwrap();
            }
        }
    }
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
