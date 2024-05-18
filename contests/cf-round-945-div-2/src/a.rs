use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [p1, p2, p3] = read.i32s();
    if (p1 + p2 + p3) % 2 == 1 {
        emitln!(write, -1);
        return;
    }
    let total = (p1 + p2 + p3) / 2;
    let wins = ((p3 - p2 - p1) / 2).max(0);
    let draws = total - wins;
    emitln!(write, draws);
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
        assert_trimmed_eq!(&run_solver(solve_case, "0 1 9"), "1");
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        0 0 0
        0 1 1
        1 1 1
        1 1 2
        3 3 3
        3 4 5
        1 1 10
        "), "\
        0
        1
        -1
        2
        -1
        6
        2");
    }
}
