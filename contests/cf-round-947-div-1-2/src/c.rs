use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_windows::IterutilsWindows;
use contest_lib_rs::sort_array::sort_array;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_u32(n);

    if a.len() <= 2 {
        a.sort_unstable();
        emitln!(write, a[0]);
        return;
    }
    let ans = a.iter().copied().array_windows().map(|v| {
        let [_, y, _] = sort_array(v);
        y
    }).max().unwrap();
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
        assert_trimmed_eq!(&run_solver(solve_case, "5  4 1 2 3 5"), "3");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        2
        1 2
        5
        1 2 3 4 5
        "), "\
        1
        4");
    }
}
