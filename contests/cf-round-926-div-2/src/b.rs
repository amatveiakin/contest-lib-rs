use contest_lib_rs::int_ext::IntegerExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.u32s();
    let mut ans = k.div_up(2);
    if k == 4*n - 2 {
        ans += 1;
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
        assert_trimmed_eq!(&run_solver(solve_case, "2 5"), "3");
        assert_trimmed_eq!(&run_solver(solve_case, "2 6"), "4");
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        3 4
        3 3
        3 10
        3 9
        4 7
        7 11
        2 3
        "), "\
        2
        2
        6
        5
        4
        6
        2");
    }
}
