use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k, x] = read.i64s();
    let ks = k * (k + 1) / 2;
    let ans = ks <= x && x <= ks + (n - k) * k;
    emitln!(write, ans.YESNO());
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
        12
        5 3 10
        5 3 3
        10 10 55
        6 5 20
        2 1 26
        187856 87856 2609202300
        200000 190000 19000000000
        28 5 2004
        2 2 2006
        9 6 40
        47202 32455 613407217
        185977 145541 15770805980
        "), "\
        YES
        NO
        YES
        YES
        NO
        NO
        YES
        NO
        NO
        NO
        YES
        YES");
    }
}
