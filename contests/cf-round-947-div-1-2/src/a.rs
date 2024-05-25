use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);

    let mut inv = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        if a[i] > a[j] {
            inv += 1;
        }
    }
    emitln!(write, (inv <= 1).YesNo());
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
        6
        1 1 4 5 1 4
        5
        7 9 2 2 3
        3
        1 2 3
        "), "\
        No
        Yes
        Yes");
    }
}
