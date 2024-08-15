use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::point_2d::PointReading;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let c = (0..n).map(|_| read.p2_i64()).collect_vec();
    let s = read.p2_i64();
    let t = read.p2_i64();

    let st = s.l2_dist_sqr(t);
    for c in c {
        let ct = c.l2_dist_sqr(t);
        if ct <= st {
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
        7
        3
        2 5
        2 14
        10 13
        4 9 9 7
        3
        10 11
        6 9
        12 12
        14 13 4 8
        1
        5 7
        12 6 11 13
        2
        1000000000 2
        2 1000000000
        1 1 2 2
        1
        999999998 1000000000
        999999999 999999999 1 1
        1
        1000000000 1
        1 1000000000 1 1
        10
        989237121 2397081
        206669655 527238537
        522705783 380636165
        532545346 320061691
        207818728 199485303
        884520552 315781807
        992311437 802563521
        205138355 324818663
        223575704 395073023
        281560523 236279118
        216941610 572010615 323956540 794523071"), "\
        YES
        NO
        YES
        YES
        YES
        NO
        YES");
    }
}
