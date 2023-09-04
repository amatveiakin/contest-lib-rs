use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [mut m, k, a1, ak] = read.u32s();
    let nk = std::cmp::min(ak, m / k);
    let mut fancy = 0;
    m -= nk * k;
    let old_m = m;
    let n1 = std::cmp::min(a1, m);
    m -= n1;
    if old_m / k != m / k {
        fancy += (m + k - 1) / k;
    } else {
        fancy += m / k + m % k;
    }
    emitln!(write, fancy);
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
4
11 3 0 0
11 3 20 20
11 3 6 1
100000000 2 0 0"), "\
5
0
1
50000000");
        assert_trimmed_eq!(&run_solver(solve_case, "99 10 5 10"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "99 10 10 5"), "4");
    }
}
