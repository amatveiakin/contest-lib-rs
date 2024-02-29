use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::segment_bucket_counter::SegmentBucketCounter;
use contest_lib_rs::segment_tree::new_sum_tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, q] = read.usizes();
    let c = read.vec_u64(n);
    let o = c.iter().map(|&x| x == 1).collect_vec();
    let mut sums = new_sum_tree(&c);
    let oc = SegmentBucketCounter::new(&o);
    for _ in 0..q {
        let [l, r] = read.u32s().from1b();
        if l == r {
            emitln!(write, "NO");
            continue;
        }
        let min = ((r - l + 1) + oc.count(true, l..=r)) as u64;
        let sum = sums.get(l..=r);
        emitln!(write, (min <= sum).yesno());
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
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "1 2  1  1 1  1 1"), "NO\nNO");
        assert_trimmed_eq!(&run_solver(solve_case, "2 1  1 1  1 2"), "NO");
        assert_trimmed_eq!(&run_solver(solve_case, "2 1  1 2  1 2"), "YES");
        assert_trimmed_eq!(&run_solver(solve_case, "2 1  2 2  1 2"), "YES");
        assert_trimmed_eq!(&run_solver(solve_case, "3 1  1 1 2  1 3"), "NO");
        assert_trimmed_eq!(&run_solver(solve_case, "3 1  1 1 3  1 3"), "YES");
        assert_trimmed_eq!(&run_solver(solve_case, "3 1  1 2 2  1 3"), "YES");
        assert_trimmed_eq!(&run_solver(solve_case, "4 1  1 1 1 3  1 4"), "NO");
        assert_trimmed_eq!(&run_solver(solve_case, "4 1  1 1 1 4  1 4"), "YES");
        assert_trimmed_eq!(&run_solver(solve_case, "4 1  1 1 2 2  1 4"), "YES");
        assert_trimmed_eq!(&run_solver(solve, "\
        1
        5 4
        1 2 1 4 5
        1 5
        4 4
        3 4
        1 3
        "), "\
        YES
        NO
        YES
        NO");
    }
}
