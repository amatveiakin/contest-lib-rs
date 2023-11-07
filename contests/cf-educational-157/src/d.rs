use std::collections::HashSet;

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n - 1);
    let mut b = 0;
    let mut bs = vec![b];
    for i in 0..(n - 1) {
        b ^= a[i];
        bs.push(b);
    }
    if bs.iter().any(|&b| b >= n as u32) {
        let mut mis: HashSet<_> = (0..(n as u32)).collect();
        for b in &bs {
            mis.remove(&b);
        }
        let extra = bs.iter().filter(|&&b| b >= n as u32).collect_vec();
        let mut fix = 0;
        for i in 0..20 {
            let mask = 1 << i;
            if extra.iter().filter(|&&x| x & mask != 0).count() != mis.iter().filter(|&&x| x & mask != 0).count() {
                fix |= mask;
            }
        }
        for b in bs.iter_mut() {
            *b ^= fix;
        }
    }
    emitln!(write, bs);
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
        assert_trimmed_eq!(&run_solver(solve, "4  2 1 2"), "0 2 3 1");
        assert_trimmed_eq!(&run_solver(solve, "6  1 6 1 4 1"), "2 3 5 4 0 1");
        // assert_trimmed_eq!(&run_solver(solve, "6  4 2 1 6 3"), "");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
