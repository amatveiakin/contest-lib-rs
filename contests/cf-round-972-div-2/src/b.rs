use std::collections::BTreeSet;

use contest_lib_rs::base_one::{BaseOneConversion, IteratorBaseOneConversion};
use contest_lib_rs::btreeset_util::OrderedSetNeighborValues;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.u32();
    let [m, q] = read.usizes();
    let b: BTreeSet<_> = read.vec_u32(m).into_iter().from1b().collect();
    let a = read.vec_u32(q).from1b();
    for x in a {
        match (b.prev_value(&x), b.next_value(&x)) {
            (Some(y), Some(z)) => {
                emitln!(write, (z - y) / 2);
            }
            (Some(y), None) => {
                emitln!(write, n - y - 1);
            }
            (None, Some(z)) => {
                emitln!(write, z);
            }
            (None, None) => {
                unreachable!();
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
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        10 2 1
        1 4
        2
        8 2 1
        3 6
        1
        8 2 1
        3 6
        8"), "\
        1
        2
        2");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        8 1 1
        6
        3
        10 3 3
        1 4 8
        2 3 10"), "\
        5
        1
        1
        2");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
