use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_u32(n);

    a.sort_unstable();
    let x = a[0];
    let mut y = None;
    for i in 1..n {
        if a[i] % x != 0 {
            if let Some(y) = y {
                if a[i] % y != 0 {
                    emit!(write, "No\n");
                    return;
                }
            } else {
                y = Some(a[i]);
            }
        }
    }
    emit!(write, "Yes\n");
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
        4
        3
        7 3 8
        5
        7 1 9 3 5
        5
        4 12 2 6 3
        5
        7 49 9 3 1000000000
        "), "\
        No
        Yes
        Yes
        No");
    }
}
