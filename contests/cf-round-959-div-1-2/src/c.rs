use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let x = read.i64();
    let a = read.vec_i64(n);

    let mut s = 0i64;
    let mut t = CountingSet::new();
    let mut ans = n * (n + 1) / 2;
    for v in a {
        s += v;
        t.push(v - s);
        while t.last().unwrap() + s > x {
            let (_, c) = t.pop_last_group().unwrap();
            ans -= c;
            t.push_multiple(-s, c);
        }
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
        5
        4 2
        1 1 1 1
        3 2
        1 2 3
        1 6
        10
        6 3
        1 2 1 4 3 8
        5 999999999
        999999999 999999998 1000000000 1000000000 500000000"), "\
        8
        2
        0
        10
        7");
    }
}
