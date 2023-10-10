use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::segment_bucket_counter::SegmentBucketCounter;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    use std::cmp::Ordering::*;
    let n = read.usize();
    let a = read.vec_i32(n);
    let d = a.windows(2).map(|w| w[1].cmp(&w[0])).collect::<Vec<_>>();
    let c = SegmentBucketCounter::new(&d);
    let mut answer = c.count(Less, ..) + c.count(Equal, ..);
    for i in 1..n {
        let i = i as u32;
        answer.relax_min(
            1
            + c.count(Greater, ..(i - 1))
            + c.count(Equal, ..(i - 1))
            + c.count(Less, i..)
            + c.count(Equal, i..)
        );
    }
    emitln!(write, answer);
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
3
5
1 1 2 2 2
6
5 4 3 2 5 1
3
1 2 3
"), "\
3
2
0");
        assert_trimmed_eq!(&run_solver(solve_case, "5  1 1 1 1 1"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "3  3 2 1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "3  2 1 2"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "3  2 1 1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "6  6 5 4 3 2 1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "6  3 2 1 1 2 3"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "4  2 1 1 2"), "1");
        // assert_trimmed_eq!(&run_solver(solve_case, ""), "");
    }
}
