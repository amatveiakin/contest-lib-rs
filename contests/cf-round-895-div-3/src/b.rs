use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut m = u32::MAX;
    for _ in 0..n {
        let [d, s] = read.u32s();
        m.relax_min(d + (s - 1) / 2);
    }
    emitln!(write, m);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        1
        2 2
        3
        2 8
        4 3
        5 2
        1
        200 200
        4
        1 20
        5 9
        3 179
        100 1
        2
        10 1
        1 18
        2
        1 1
        1 2
        3
        1 3
        1 1
        1 3
        "), "\
        2
        5
        299
        9
        9
        1
        1");
    }
}
