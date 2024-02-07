use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::segment_tree::new_homogenous_tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Element {
    val: u32,
    pos: usize,
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n).into_iter().enumerate().map(|(pos, val)| Element { val, pos }).collect_vec();

    let mut mint = new_homogenous_tree(&a, Element { val: u32::MAX, pos: 0 }, |&x, &y, _| x.min(y) );
    let mut maxt = new_homogenous_tree(&a, Element { val: u32::MIN, pos: 0 }, |&x, &y, _| x.max(y) );

    let q = read.usize();
    for _ in 0..q {
        let [l, r] = read.u32s().from1b();
        let min = mint.get(l..=r);
        let max = maxt.get(l..=r);
        if min.val == max.val {
            emitln!(write, -1, -1);
        } else {
            emitln!(write, min.pos.to1b(), max.pos.to1b());
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 5
        // 1 1 2 1 1
        // 3
        // 1 5
        // 1 2
        // 1 3
        // 6
        // 30 20 20 10 10 20
        // 5
        // 1 2
        // 2 3
        // 2 4
        // 2 6
        // 3 5
        // 4
        // 5 2 3 4
        // 4
        // 1 2
        // 1 4
        // 2 3
        // 2 4
        // 5
        // 1 4 3 2 4
        // 5
        // 1 5
        // 2 4
        // 3 4
        // 3 5
        // 4 5
        // 5
        // 2 3 1 4 2
        // 7
        // 1 2
        // 1 4
        // 1 5
        // 2 4
        // 2 5
        // 3 5
        // 4 5
        // "), "\
        // 2 3
        // -1 -1
        // 1 3

        // 2 1
        // -1 -1
        // 4 2
        // 4 6
        // 5 3

        // 1 2
        // 1 2
        // 2 3
        // 3 2

        // 1 3
        // 2 4
        // 3 4
        // 5 3
        // 5 4

        // 1 2
        // 4 2
        // 1 3
        // 2 3
        // 3 2
        // 5 4
        // 5 4");
    }
}
