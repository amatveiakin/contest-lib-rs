use contest_lib_rs::point_2d::{PointReading, Point2D};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax_float::RelaxFloat;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let p = read.p2_f64();
    let a = read.p2_f64();
    let b = read.p2_f64();
    let o = Point2D::zero();
    let l = [a, b];
    let mut best_d = f64::MAX;
    for (pl, ol) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
        let mut d = (p.l2_dist(l[pl])).max(o.l2_dist(l[ol]));
        if pl != ol {
            d.relax_max(l[pl].l2_dist(l[ol]) / 2.0);
        }
        best_d.relax_min(d);
    }
    emitln!(write, best_d);
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
        assert_fuzzy_eq!(&run_solver(solve, "\
        2
        3 3
        1 0
        -1 6
        3 3
        -1 -1
        4 3
        "), "\
        3.6055512755
        3.2015621187");
    }
}
