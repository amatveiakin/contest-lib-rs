use contest_lib_rs::io::prelude::*;
use contest_lib_rs::prefix_accumulate::PrefixSum;
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_i64(n);

    let mut ans1: i64 = 0;
    for i in 0..n {
        if (a[i] > 0) == (i % 2 == 0) {
            ans1 += a.drain(i..).filter(|&x| x > 0).sum::<i64>();
            break;
        }
    }
    let sums = PrefixSum::from_iter(a.iter().copied().map(|x| x.max(0)));
    let mut ans2: i64 = 0;
    for i in 0..a.len() {
        let mut v = 0;
        if i % 2 == 0 {
            v += a[i];
        }
        v += sums.get((i as u32 + 1)..);
        ans2.relax_max(v);
    }
    emitln!(write, ans1 + ans2);
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
        4
        -4 1 -3 5
        4
        1 -2 3 -4
        3
        -1 3 -5
        1
        -1
        "), "\
        5
        4
        2
        0");
    }
}
