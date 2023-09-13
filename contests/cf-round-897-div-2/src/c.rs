use std::process::exit;

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mex::get_mex;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i32();
    let s = read.vec_i32(n as usize);
    let m = get_mex(s);
    emitln!(write, m);
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
