use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_u32(n);
    a.sort();
    let p = (n - 1) / 2;
    let m = a[p];
    let mut ans = 0;
    for i in p..n {
        if a[i] != m {
            break;
        }
        ans += 1;
    }
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
        assert_trimmed_eq!(&run_solver(solve, "\
        8
        3
        2 2 8
        4
        7 3 3 1
        1
        1000000000
        5
        5 5 5 4 5
        6
        2 1 2 3 1 4
        2
        1 2
        2
        1 1
        4
        5 5 5 5
        "), "\
        1
        2
        1
        3
        2
        1
        2
        3");
    }
}
