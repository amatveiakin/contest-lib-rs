// UNFINISHED

use contest_lib_rs::{io, emitln};
use contest_lib_rs::relax::RelaxMinMax;

fn set_cost(b: &[u32]) -> u32 {
    let mut t = 1 << 30;
    for i in 0..b.len() {
        for j in 0..i {
            t.relax_min(b[i] ^ b[j]);
        }
    }
    t
}

fn pair_cost(b: &[u32], c: &[u32]) -> u32 {
    std::cmp::min(set_cost(b), set_cost(c))
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let mut b = vec![];
    let mut c = vec![];
    for &v in &a {
        b.push(v);
        let c1 = pair_cost(&b, &c);
        b.pop();
        c.push(v);
        let c2 = pair_cost(&b, &c);
        c.pop();
        if c1 > c2 {
            b.push(v);
        } else {
            c.push(v);
        }
    }
    let answer = pair_cost(&b, &c);
    emitln!(write, answer);
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
        // assert_trimmed_eq!(&run_solver(solve, "4  1 2 3 4"), "");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
