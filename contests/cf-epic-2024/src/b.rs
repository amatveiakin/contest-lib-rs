use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let b = read.vec_u32(n);
    let mut br = b.clone();
    br.reverse();
    emitln!(write, if a == b || a == br { "Bob" } else { "Alice" })
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
        2
        2
        1 2
        1 2
        3
        1 2 3
        2 3 1"), "\
        Bob
        Alice");
        assert_trimmed_eq!(&run_solver(solve_case, "1  1  1"), "Bob");
        assert_trimmed_eq!(&run_solver(solve_case, "3  1 2 3  3 2 1"), "Bob");
        assert_trimmed_eq!(&run_solver(solve_case, "4  1 2 3 4  4 3 2 1"), "Bob");
        assert_trimmed_eq!(&run_solver(solve_case, "4  1 2 3 4  4 2 3 1"), "Alice");
    }
}
