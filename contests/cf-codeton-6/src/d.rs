use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::segment_tree::new_sum_tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.u32();
    let c = read.vec_u32(n as usize);
    let mut k = read.u32();

    let mut cf = vec![];
    let mut min = u32::MAX;
    for (i, &v) in c.iter().enumerate().rev() {
        if v < min {
            min = v;
            cf.push((i as u32, v));
        }
    }
    cf.reverse();

    let a = vec![0; n as usize];
    let mut a = new_sum_tree(&a);
    let mut start = 0;
    let mut prev_v = 0;
    let mut prev_d = u32::MAX;
    for &(i, v) in &cf {
        let d = (k / (v - prev_v)).min(prev_d);
        k -= d * (v - prev_v);
        a.update(start..=i, &d);
        start = i + 1;
        prev_v = v;
        prev_d = d;
    }
    let a = a.iter().collect_vec();
    emitln!(write, a);
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
        3
        1 2 3
        5
        2
        3 4
        7
        3
        3 2 1
        2
        6
        10 6 4 6 3 4
        7
        "), "\
        5 0 0
        2 1
        2 2 2
        2 2 2 2 2 1 ");
        assert_trimmed_eq!(&run_solver(solve_case, "3  10 15 16  37"), "3 1 1");
    }
}
