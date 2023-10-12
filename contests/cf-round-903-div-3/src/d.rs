use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::factors::factors;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let mut f = CountingSet::new();
    for x in a {
        for (a, p) in factors(x) {
            f.push_multiple(a, p as usize);
        }
    }
    for (_, p) in f.group_iter() {
        if p % n != 0 {
            emitln!(write, "NO");
            return;
        }
    }
    emitln!(write, "YES");
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
        5
        100 2 50 10 1
        3
        1 1 1
        4
        8 2 4 2
        4
        30 50 27 20
        2
        75 40
        2
        4 4
        3
        2 3 1
        "), "\
        YES
        YES
        NO
        YES
        NO
        YES
        NO");
    }
}
