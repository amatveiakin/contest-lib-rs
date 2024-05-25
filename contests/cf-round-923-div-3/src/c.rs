use std::collections::HashSet;

use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let k = read.u32();
    let a: HashSet<_> = read.vec_u32(n).into_iter().collect();
    let b: HashSet<_> = read.vec_u32(m).into_iter().collect();
    let mut oa = 0;
    let mut ob = 0;
    for x in 1..=k {
        match (a.contains(&x), b.contains(&x)) {
            (false, false) => {
                emitln!(write, "NO");
                return;
            }
            (true, false) => oa += 1,
            (false, true) => ob += 1,
            (true, true) => {}
        }
    }
    emitln!(write, (oa <= k/2 && ob <= k/2).YESNO());
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
        6
        6 5 6
        2 3 8 5 6 5
        1 3 4 10 5
        6 5 6
        2 3 4 5 6 5
        1 3 8 10 3
        3 3 4
        1 3 5
        2 4 6
        2 5 4
        1 4
        7 3 4 4 2
        1 4 2
        2
        6 4 4 2
        1 5 2
        3
        2 2 1 4 3
        "), "\
        YES
        NO
        YES
        YES
        NO
        NO");
    }
}
