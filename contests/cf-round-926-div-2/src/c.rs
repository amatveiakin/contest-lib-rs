use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [k, x, a] = read.u64s();
    let mut s = 0;
    for _ in 0..=x {
        let b = s / (k - 1) + 1;
        s += b;
        if s > a {
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "2 100 1000000000"), "NO");
        assert_trimmed_eq!(&run_solver(solve, "\
        9
        2 1 7
        2 1 1
        2 3 15
        3 3 6
        4 4 5
        5 4 7
        4 88 1000000000
        25 69 231
        13 97 18806
        "), "\
        YES
        NO
        YES
        NO
        NO
        YES
        NO
        NO
        NO");
    }
}
