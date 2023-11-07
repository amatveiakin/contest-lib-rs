use contest_lib_rs::bitset::Bitset;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i32();
    let k = read.i32();
    let b = read.vec_i32(n as usize);

    let mut p = n - 1;
    let mut s = 0;
    let mut visited = Bitset::new(n as usize);
    while s < k && !visited.get(p as usize) {
        visited.set(p as usize, true);
        let sh = b[p as usize];
        if sh > n {
            emitln!(write, "No");
            return;
        }
        p = (p - sh).rem_euclid(n);
        s += 1;
    }
    emitln!(write, "Yes");
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
        assert_trimmed_eq!(&run_solver(solve_case, "5 5  6 1 1 1 1"), "No");
        assert_trimmed_eq!(&run_solver(solve_case, "5 5  5 1 1 1 1"), "Yes");
        assert_trimmed_eq!(&run_solver(solve_case, "5 4  6 1 1 1 1"), "Yes");
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        5 3
        4 3 3 2 3
        3 100
        7 2 1
        5 5
        6 1 1 1 1
        1 1000000000
        1
        8 48
        9 10 11 12 13 14 15 8
        2 1
        1 42
        "), "\
        Yes
        Yes
        No
        Yes
        Yes
        No");
    }
}
