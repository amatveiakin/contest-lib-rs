use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.u32();
    if n <= 6 || n == 9 {
        emitln!(write, "NO");
    } else {
        emitln!(write, "YES");
        if n % 3 == 0 {
            emitln!(write, 1, 4, n - 5);
        } else {
            emitln!(write, 1, 2, n - 3);
        }
    }
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 10
        // 4
        // 15
        // 9
        // "), "\
        // YES
        // 4 5 1
        // NO
        // YES
        // 2 8 5
        // NO");
    }
}
