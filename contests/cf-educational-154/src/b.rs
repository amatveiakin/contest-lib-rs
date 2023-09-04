use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let a = read.word().chars().collect::<Vec<_>>();
    let b = read.word().chars().collect::<Vec<_>>();
    let n = a.len();
    for i in 0..(n - 1) {
        if a[i] == '0' && a[i+1] == '1' && b[i] == '0' && b[i+1] == '1' {
            emitln!(write, "YES");
            return;
        }
    }
    emitln!(write, "NO");
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
01010001
01110101
01001
01001
000101
010111
00001
01111
011
001
001001
011011
010001
011011"), "\
YES
YES
YES
NO
NO
NO
YES");
    }
}
