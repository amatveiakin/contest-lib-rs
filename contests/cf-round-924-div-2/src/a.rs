use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [a, b] = read.u32s();
    let ans =
        (a % 2 == 0 && a / 2 != b) ||
        (b % 2 == 0 && b / 2 != a);
    emitln!(write, ans.yesno());
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
        1 1
        2 1
        2 6
        3 2
        2 2
        2 4
        6 3
        "), "\
        NO
        NO
        YES
        YES
        YES
        YES
        NO");
    }
}
