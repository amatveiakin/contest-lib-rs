// UNFINISHED

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;

type ModNum = ModNumber::<999_999_893>;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.u32();

    // let a = ModNum::from(2);
    // let b = ModNum::from(4);
    // let c = ModNum::from(2);
    // let d = ModNum::from(1);

    let a = ModNum::from(2);
    let b = ModNum::from(2);
    let c = ModNum::from(2);
    let d = ModNum::from(0);

    let pq = (b - d) / (c * c - ModNum::from(2) * d * d);
    emitln!(write, pq);
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
