use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let k = read.u32();
    let a = read.vec_u32(n);
    emitln!(write, a.contains(&k).yesno());
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
        5 4
        1 4 3 4 1
        4 1
        2 3 4 4
        5 6
        43 5 60 4 2
        2 5
        1 5
        4 1
        5 3 3 1
        1 3
        3
        5 3
        3 4 1 5 5
        "), "\
        YES
        NO
        NO
        YES
        YES
        YES
        YES");
    }
}
