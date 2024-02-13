use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let [x, y] = read.u32s();
    let a = read.vec_u32(n);
    let mut m = CountingSet::new();
    let mut ans: usize = 0;
    for v in a {
        let r = (v % x, v % y);
        ans += m.count(&((x - r.0) % x, r.1));
        m.push(r);
    }
    emitln!(write, ans);
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
        7
        6 5 2
        1 2 7 4 9 6
        7 9 5
        1 10 15 3 8 12 15
        9 4 10
        14 10 2 2 11 11 13 5 6
        9 5 6
        10 7 6 7 9 7 7 10 10
        9 6 2
        4 9 7 1 2 2 13 3 15
        9 2 3
        14 6 1 15 12 15 8 2 15
        10 5 7
        13 3 3 2 12 11 3 7 13 14
        "), "\
        2
        0
        1
        3
        5
        7
        0");
    }
}
