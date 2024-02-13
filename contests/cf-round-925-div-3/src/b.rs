use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.u64();
    let a = read.vec_u64(n as usize);
    let avg = a.iter().sum::<u64>() / n;
    let mut s = 0;
    for i in 0..n {
        s += a[i as usize];
        if s < avg * (i + 1) {
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
        6
        1
        43
        2
        1 3
        5
        4 5 2 1 3
        3
        1 2 3
        7
        4 5 5 0 6 4 4
        7
        6 5 5 1 3 4 4
        "), "\
        YES
        NO
        YES
        NO
        NO
        YES");
    }
}
